mod db;

use std::process;

use db::{ClientConnection, MongoDBData, Operation};
use mongodb::bson::Document;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = "mongodb://localhost:27017";
    let client = ClientConnection::new(uri).await?;

    loop {
        println!("Enter operation:");
        println!("1. List databases");
        println!("2. Read");
        println!("3. Insert");
        println!("4. Update");
        println!("5. Delete");
        println!("6. Exit\n");

        let mut input = String::new();

        std::io::stdin().read_line(&mut input)?;

        let operation = match input.trim() {
            "1" => Operation::ListDatabase,
            "2" => Operation::Read,
            "3" => Operation::Insert,
            "4" => Operation::Update,
            "5" => Operation::Delete,
            "6" => {
                eprintln!("Exiting program");
                process::exit(0);
            }
            _ => {
                println!("Invalid input");
                continue;
            }
        };

        let data: Document = match &operation {
            Operation::ListDatabase => {
                client.list_database_names(None, None).await?;
                continue;
            }
            _ => MongoDBData::new()
                .get_data(&operation)
                .expect("Failed to get data"),
        };

        client.start(data, operation).await?;
    }
}
