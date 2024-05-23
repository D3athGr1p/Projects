use std::error::Error;

use reqwest::Client;

pub enum Operation {
    Create,
    Read,
    Update,
    Delete,
    Quit,
}

impl Operation {
    pub fn new(option: String) -> Option<Operation> {
        let operation = match option.as_str() {
            "w" => Some(Operation::Create),
            "r" => Some(Operation::Read),
            "u" => Some(Operation::Update),
            "d" => Some(Operation::Delete),
            "q" => Some(Operation::Quit),
            _ => None,
        };
        operation
    }

    pub fn create(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn read(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn update(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn delete(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn quit(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub struct RestClient {
    client: Client,
    base_url: String,
    operation: Option<Operation>,
}

impl RestClient {
    pub fn new(base_url: String) -> RestClient {
        RestClient {
            client: Client::new(),
            base_url: base_url,
            operation: None,
        }
    }

    pub fn set_operation(&mut self, operation: Operation) {
        self.operation = Some(operation);
    }
}
