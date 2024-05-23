mod client;

use std::{io, process};

use client::{Operation, RestClient};

fn main() {
    let connectin_string = "http://127.0.0.1:3000/api".to_string();
    let mut client = RestClient::new(connectin_string);

    loop {
        let mut options = String::new();

        println!(
            "
Press w to write
Press r to read
Press u to update
Press d to delete
Press q to quit
        "
        );

        io::stdin()
            .read_line(&mut options)
            .expect("Failed to read line");

        let options = options.trim().to_lowercase();

        let options = Operation::new(options);

        if options.as_ref().is_none() {
            println!("Invalid option");
            process::exit(1);
        }

        client.set_operation(options.unwrap());
    }
}
