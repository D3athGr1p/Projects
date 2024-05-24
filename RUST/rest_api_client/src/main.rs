mod client;

use client::{Operation, RestClient};
use std::fmt;
use std::process;
use std::{error::Error as Er, io};

#[derive(Debug)]
struct EmptyValue;
impl fmt::Display for EmptyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Empty value")
    }
}

impl Er for EmptyValue {}

#[derive(Debug)]
struct InputData {
    id: u32,
    name: Option<String>,
    value: Option<String>,
}

impl InputData {
    pub fn new() -> InputData {
        InputData {
            id: 0,
            name: None,
            value: None,
        }
    }
    fn read_input(&self, input_field: &str) -> Result<String, Box<dyn Er>> {
        println!("Please enter {} :", input_field);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        input = input.trim().to_owned();

        if input.is_empty() {
            return Err(Box::new(EmptyValue));
        } else {
            return Ok(input);
        }
    }

    pub fn set_id(&mut self) -> Result<(), Box<dyn Er>> {
        let id = self.read_input("id")?;

        self.id = id.parse()?;

        Ok(())
    }

    pub fn set_name(&mut self) -> Result<(), Box<dyn Er>> {
        self.name = Some(self.read_input("name")?);

        Ok(())
    }

    pub fn set_value(&mut self) -> Result<(), Box<dyn Er>> {
        self.value = Some(self.read_input("value")?);

        Ok(())
    }

    pub fn set_data(&mut self) -> Result<(), Box<dyn Er>> {
        self.set_id()?;
        self.set_name()?;
        self.set_value()?;

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let connectin_string = "http://127.0.0.1:3000/api".to_string();
    let mut client = RestClient::new(connectin_string);

    loop {
        let mut options = String::new();

        println!(
            "Press w to write
Press r to read
Press u to update
Press d to delete
Press q to quit"
        );

        io::stdin()
            .read_line(&mut options)
            .expect("Failed to read line");

        let options = options.trim().to_lowercase();

        let options = Operation::new(options);

        if options.as_ref().is_none() {
            println!("Invalid option");
            continue;
        }

        client.set_operation(options.unwrap());

        let mut data = InputData::new();

        match data.set_data() {
            Ok(_) => println!("Data set successfully\n\n"),
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        };

        client = client.start_operation(data).await.unwrap_or_else(|error| {
            eprintln!("Operation failed");
            eprintln!("Exiting current program");
            eprintln!("Found error {:?}", error);
            process::exit(0);
        });
    }
}
