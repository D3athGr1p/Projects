use super::InputData;
use reqwest::Client;
use std::{error::Error, process};

#[derive(Debug)]
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
}

#[derive(Debug)]
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

    pub async fn start_operation(&mut self, data: InputData) -> Result<RestClient, Box<dyn Error>> {
        let _ = match self.operation.as_ref().unwrap() {
            Operation::Create => self.ops(data, Operation::Create).await?,
            Operation::Read => self.ops(data, Operation::Read).await?,
            Operation::Update => self.ops(data, Operation::Update).await?,
            Operation::Delete => self.ops(data, Operation::Delete).await?,
            Operation::Quit => self.quit().await?,
        };

        Ok(RestClient {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            operation: None,
        })
    }

    pub async fn ops(&self, data: InputData, operation: Operation) -> Result<(), Box<dyn Error>> {
        let (url, json_data) = match operation {
            Operation::Read => (
                format!("{}/read", self.base_url),
                format!(r#"{{"key":"{}"}}"#, data.id),
            ),
            Operation::Create => (
                format!("{}/create", self.base_url),
                format!(
                    r#"{{"key":"{}","name":"{}","value":"{}"}}"#,
                    data.id,
                    data.name.unwrap(),
                    data.value.unwrap()
                ),
            ),
            Operation::Update => (
                format!("{}/update", self.base_url),
                format!(
                    r#"{{"key":"{}","name":"{}","value":"{}"}}"#,
                    data.id,
                    data.name.unwrap(),
                    data.value.unwrap()
                ),
            ),
            Operation::Delete => (
                format!("{}/delete", self.base_url),
                format!(r#"{{"key":"{}"}}"#, data.id),
            ),
            Operation::Quit => unreachable!(),
        };

        self.send_request(url, json_data).await?;

        Ok(())
    }

    pub async fn send_request(&self, url: String, json_data: String) -> Result<(), Box<dyn Error>> {
        let resp = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(json_data)
            .send()
            .await?;

        println!("Status Code: {}", resp.status());

        let response_body = resp.text().await?;

        println!("Response body: \n{}", response_body);

        Ok(())
    }

    pub async fn quit(&self) -> Result<(), Box<dyn Error>> {
        process::exit(0);
    }
}
