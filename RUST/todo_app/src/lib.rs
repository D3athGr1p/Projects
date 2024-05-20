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
        let query: String = "Pending: ".to_owned() + &query;
        Ok(file.write_all(query.as_bytes())?)
    }

    pub fn remove(query: String, file: &mut File) -> Result<(), Box<dyn Error>> {
        let mut updated_contents = String::new();
        let content = BufReader::new(&mut *file);
        for line in content.lines() {
            let line = line?;
            if !line.contains(&query) {
                updated_contents.push_str(&line);
                updated_contents.push('\n');
            }
        }
        file.set_len(0)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        file.write_all(updated_contents.as_bytes())?;

        Ok(())
    }

    pub fn done(query: String, file: &mut File) -> Result<(), Box<dyn Error>> {
        let mut updated_contents = String::new();
        let content = BufReader::new(&mut *file);
        for line in content.lines() {
            let mut line = line?;
            if line.contains(&query) {
                line = line.replace("Pending", "Done");
            }
            updated_contents.push_str(&line);
            updated_contents.push('\n');
        }
        file.set_len(0)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        file.write_all(updated_contents.as_bytes())?;

        Ok(())
    }

    pub fn read(file: File) -> Result<(), Box<dyn Error>> {
        let content = BufReader::new(file);
        for line in content.lines() {
            println!("{}", line?);
        }

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
        State::Add => Ok(State::add(config.query, &mut file)?),
        State::Remove => Ok(State::remove(config.query, &mut file)?),
        State::Done => Ok(State::done(config.query, &mut file)?),
        State::Read => Ok(State::read(file)?),
    }
}
