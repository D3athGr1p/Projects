use encryptor_decryptor::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 6 {
        println!("Invalid number of arguments");
        process::exit(0);
    }

    let args: Vec<String> = args.iter().map(|s| s.trim().to_owned()).collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Error parsing arguments: {}", error);
        process::exit(1);
    });

    encryptor_decryptor::run(&config);
}
