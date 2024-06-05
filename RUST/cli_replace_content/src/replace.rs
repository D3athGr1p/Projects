use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::*;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    eprintln!(
        "{} - replace a string with a new string",
        "Find and Replace".green()
    );
    eprintln!("Usage: <target string> <replace string> <Input File> <Output File>");
}

fn replace(target: &str, replace: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data, replace).to_string())
}

fn read_and_write(args: &Arguments) {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!(
                "{}: Failed to read input file: {} with error: {}",
                "Error".red().bold(),
                &args.input_file,
                err
            );
            std::process::exit(1);
        }
    };
    let data = match replace(&args.pattern, &args.replace, &data) {
        Ok(data) => data,
        Err(err) => {
            eprintln!(
                "{}: Failed to replace string: {} with error: {}",
                "Error".red().bold(),
                &args.pattern,
                err
            );
            std::process::exit(1);
        }
    };

    match fs::write(&args.output_file, &data) {
        Ok(_) => {}
        Err(err) => {
            eprintln!(
                "{}: Failed to write output file: {} with error: {}",
                "Error".red().bold(),
                &args.output_file,
                err
            );
            std::process::exit(1);
        }
    }
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_help();
        eprintln!(
            "{} wrong number of arguments, Expected 4, got {}",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    Arguments {
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(),
    }
}

pub fn run() {
    let args = parse_args();
    read_and_write(&args);
}
