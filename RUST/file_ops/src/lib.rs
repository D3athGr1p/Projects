use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Debug)]
pub struct Config {
    file_operation: Option<FileOperations>,
    from: Option<String>,
    to: Option<String>,
}

#[derive(Debug)]
pub struct ParsedConfig {
    file_operation: FileOperations,
    from: String,
    to: String,
}

#[derive(Debug)]
pub enum FileOperations {
    Copy,
    Move,
    Rename,
    Delete,
    Help,
}

impl FileOperations {
    pub fn operation(config: ParsedConfig) -> Result<(), Box<dyn Error>> {
        println!(
            "{:?}ing file from {} to {}",
            config.file_operation, config.from, config.to
        );

        match config.file_operation {
            FileOperations::Copy => {
                fs::copy(config.from, config.to).unwrap_or_else(|error| {
                    eprintln!("{}", error);
                    process::exit(0)
                });
            }
            FileOperations::Move => fs::rename(config.from, config.to)?,
            FileOperations::Rename => fs::rename(config.from, config.to)?,
            FileOperations::Delete => fs::remove_file(config.from)?,
            _ => unreachable!(),
        };

        Ok(())
    }
}

impl Config {
    pub fn new(args: &Vec<String>) -> Config {
        let config = parse_args(args).unwrap_or_else(|error| {
            eprintln!("Error parsing arguments {}", error);
            process::exit(0)
        });

        match config.file_operation.as_ref().unwrap() {
            FileOperations::Help => {
                print_help();
                process::exit(0)
            }
            _ => match config.from.as_ref() {
                None => {
                    eprintln!("File not found provide file using -F of --File");
                    print_help();
                    process::exit(0)
                }
                _ => {
                    let file_path: String = config.from.clone().unwrap();
                    let is_present = Path::new(&file_path).exists();
                    if !is_present {
                        println!("File not found: {}", file_path);
                        process::exit(0)
                    }
                }
            },
        }
        config
    }

    pub fn parse_config(config: Config) -> ParsedConfig {
        let file_operation = match config.file_operation.as_ref().unwrap() {
            FileOperations::Copy => FileOperations::Copy,
            FileOperations::Move => FileOperations::Move,
            FileOperations::Rename => FileOperations::Rename,
            FileOperations::Delete => FileOperations::Delete,
            FileOperations::Help => FileOperations::Help,
        };

        let from = match config.from.as_ref() {
            Some(_) => config.from.unwrap(),
            None => {
                eprintln!("No from Found");
                process::exit(0)
            }
        };

        let to = match config.to.as_ref() {
            Some(_) => config.to.unwrap(),
            None => match config.file_operation.as_ref().unwrap() {
                FileOperations::Delete => "".to_owned(),
                _ => {
                    eprintln!("No to Found");
                    process::exit(0)
                }
            },
        };

        ParsedConfig {
            file_operation,
            from,
            to,
        }
    }
}

fn print_help() {
    println!(
        "
Usage: cargo run [OPTION]
    -h, --help      : print help message
    -F, --file      : set the file to copy/move/rename/delete
    -m, --move      : move the file
    -c, --copy      : copy the file
    -r, --rename    : rename the file
    -d, --delete    : delete the file
    -f, --from      : set the file from from location
    -t, --to        : set the file to to location
    "
    );
}

pub fn parse_args(args: &Vec<String>) -> Result<Config, &'static str> {
    let mut i = 1;

    let mut config = Config {
        file_operation: None,
        from: None,
        to: None,
    };

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => config.file_operation = Some(FileOperations::Help),
            "-m" | "--move" => config.file_operation = Some(FileOperations::Move),
            "-c" | "--copy" => config.file_operation = Some(FileOperations::Copy),
            "-r" | "--rename" => config.file_operation = Some(FileOperations::Rename),
            "-d" | "--delete" => config.file_operation = Some(FileOperations::Delete),
            "-f" | "--from" | "-F" | "--File" => {
                if i + 1 < args.len() {
                    config.from = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    return Err("Missing value for -f or --from");
                }
            }
            "-t" | "--to" => {
                if i + 1 < args.len() {
                    config.to = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    return Err("Missing value for -t or --to");
                }
            }
            _ => return Err("Unknown argument"),
        }
        i += 1;
    }

    Ok(config)
}
