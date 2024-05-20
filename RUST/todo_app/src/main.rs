use std::{env, process};
use todo_app::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    todo_app::run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });
}
