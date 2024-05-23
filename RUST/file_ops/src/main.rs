use file_ops::{Config, FileOperations};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args: Vec<String> = args.iter().map(|s| s.trim().to_owned()).collect();

    let config = Config::new(&args);

    let config = Config::parse_config(config);

    FileOperations::operation(config).unwrap_or_else(|error| {
        eprintln!("Error {}", error);
        process::exit(0)
    });
}
