use mongodb::{
    bson::{doc, Document},
    options::ListDatabasesOptions,
    Client, Collection,
};
use std::{error::Error, io};

pub enum Operation {
    ListDatabase,
    Read,
    Insert,
    Update,
    Delete,
}

pub struct MongoDBData;

impl MongoDBData {
    pub fn new() -> MongoDBData {
        MongoDBData
    }

    pub fn get_data(&self, operations: &Operation) -> Result<Document, Box<dyn Error>> {
        match operations {
            Operation::Read | Operation::Delete => Ok(doc! {
                "title": self.get_title().unwrap().as_str(),
            }),
            Operation::Insert | Operation::Update => Ok(doc! {
                "title": self.get_title().unwrap().as_str(),
                "body": self.get_body().unwrap().as_str(),
                "author" : self.get_author().unwrap().as_str(),
            }),
            _ => unreachable!(),
        }
    }

    fn get_title(&self) -> Result<String, Box<dyn Error>> {
        let mut title = String::new();
        println!("Please enter a title");
        io::stdin()
            .read_line(&mut title)
            .expect("Failed to read title");

        Ok(title)
    }

    fn get_body(&self) -> Result<String, Box<dyn Error>> {
        let mut body = String::new();
        println!("Please enter a body");
        io::stdin()
            .read_line(&mut body)
            .expect("Failed to read body");

        Ok(body)
    }

    fn get_author(&self) -> Result<String, Box<dyn Error>> {
        let mut author = String::new();
        println!("Please enter a author\n\n");
        io::stdin()
            .read_line(&mut author)
            .expect("Failed to read author");

        Ok(author)
    }
}

pub struct ClientConnection {
    client: Client,
}

impl ClientConnection {
    pub async fn new(uri: &str) -> Result<ClientConnection, Box<dyn Error>> {
        let client = Client::with_uri_str(uri).await?;
        Ok(ClientConnection { client: client })
    }

    pub async fn list_database_names(
        &self,
        filter: Option<Document>,
        options: Option<ListDatabasesOptions>,
    ) -> Result<(), Box<dyn Error>> {
        println!("Databases:");
        for name in self.client.list_database_names(filter, options).await? {
            println!("- {}", name);
        }
        println!("\n");
        Ok(())
    }

    pub async fn start(&self, data: Document, operation: Operation) -> Result<(), Box<dyn Error>> {
        let collection = self.get_collections("orderbook", "orders").await?;
        match operation {
            Operation::Read => self.read(data, collection).await?,
            Operation::Insert => self.insert(data, collection).await?,
            Operation::Update => self.update(data, collection).await?,
            Operation::Delete => self.delete(data, collection).await?,
            _ => unreachable!(),
        };
        Ok(())
    }

    async fn get_collections(
        &self,
        db_name: &str,
        collection_name: &str,
    ) -> Result<Collection<Document>, Box<dyn Error>> {
        Ok(self
            .client
            .database(db_name)
            .collection::<Document>(collection_name))
    }

    async fn insert(
        &self,
        data: Document,
        collection: Collection<Document>,
    ) -> Result<(), Box<dyn Error>> {
        let _ = collection.insert_one(data, None).await?;

        Ok(())
    }

    async fn read(
        &self,
        data: Document,
        collection: Collection<Document>,
    ) -> Result<(), Box<dyn Error>> {
        let data = collection.find_one(data, None).await?;

        let data = match data {
            Some(data) => data,
            None => {
                println!("No data found\n");
                return Ok(());
            }
        };

        println!("data = {:?}\n", data);

        Ok(())
    }

    async fn update(
        &self,
        data: Document,
        collection: Collection<Document>,
    ) -> Result<(), Box<dyn Error>> {
        let query = doc! {
            "title": data.get("title").unwrap().as_str().unwrap()
        };

        let updates = doc! {
            "$set": data
        };

        let _ = collection.update_one(query, updates, None).await?;

        Ok(())
    }

    async fn delete(
        &self,
        data: Document,
        collection: Collection<Document>,
    ) -> Result<(), Box<dyn Error>> {
        let _ = collection.delete_one(data, None).await?;

        Ok(())
    }
}
