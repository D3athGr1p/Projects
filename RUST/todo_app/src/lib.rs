use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Seek, Write},
};

const FILENAME: &str = "./TODO.txt";

pub enum State {
    Add,
    Remove,
    Done,
    Read,
}

impl State {
    pub fn add(query: String, file: &mut File) -> Result<(), Box<dyn Error>> {
        let query: String = format!("Pending: {}\n", query);
        Ok(file.write_all(query.as_bytes())?)
    }

    pub fn read(file: File) -> Result<(), Box<dyn Error>> {
        let content = BufReader::new(file);
        for line in content.lines() {
            println!("{}", line?);
        }

        Ok(())
    }

    pub fn remove_or_done(
        state: State,
        query: String,
        file: &mut File,
        replace: Option<(&str, &str)>,
    ) -> Result<(), Box<dyn Error>> {
        let mut updated_contents = String::new();
        let content = BufReader::new(&*file);
        for line in content.lines() {
            let mut line: String = line?;
            match state {
                State::Done => {
                    if let Some((old, new)) = replace {
                        if line.contains(&query) {
                            line = line.replace(old, new);
                        }
                        updated_contents.push_str(&line);
                        updated_contents.push('\n');
                    }
                }
                State::Remove => {
                    if !line.contains(&query) {
                        updated_contents.push_str(&line);
                        updated_contents.push('\n');
                    }
                }
                _ => return Err(format!("invalid operation: {}", query).into()),
            }
        }
        file.set_len(0)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        file.write_all(updated_contents.as_bytes())?;

        Ok(())
    }
}

pub struct Config {
    query: String,
    operation: State,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        if args.len() > 4 {
            return Err("too many arguments");
        }
        let query = args[1].trim();
        let operation = args[2].trim().to_lowercase();
        let operation: State = match operation.as_str() {
            "add" => State::Add,
            "remove" => State::Remove,
            "done" => State::Done,
            "read" => State::Read,
            _ => return Err("invalid operation"),
        };

        Ok(Config {
            query: query.to_string(),
            operation: operation,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .write(true)
        .create(true)
        .open(FILENAME)?;

    match &config.operation {
        State::Add => State::add(config.query, &mut file)?,
        State::Remove => State::remove_or_done(
            State::Remove,
            config.query,
            &mut file,
            Some(("Pending", "Done")),
        )?,
        State::Done => State::remove_or_done(
            State::Done,
            config.query,
            &mut file,
            Some(("Pending", "Done")),
        )?,
        State::Read => State::read(file)?,
    };

    Ok(())
}
