use anyhow::Result;
use ethers::prelude::*;
use ethers::utils::hex;
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

#[derive(Serialize, Deserialize, Debug)]
struct BlockData {
    number: U64,
    block: serde_json::Value,
    transactions: Vec<TransactionData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TransactionData {
    hash: H256,
    from: Address,
    to: Option<Address>,
    value: U256,
    input: String,
    decoded_input: Option<serde_json::Value>,
}

async fn fetch_and_store_block(
    client: Arc<Client>,
    provider: Arc<Provider<Http>>,
    block_number: U64,
    semaphore: Arc<Semaphore>,
) -> Result<()> {
    let _permit = semaphore.acquire().await.unwrap();

    if let Some(block) = provider.get_block_with_txs(block_number).await? {
        let block_json = serde_json::to_value(&block)?;

        let transactions: Vec<TransactionData> = block
            .transactions
            .iter()
            .map(|tx| {
                let decoded_input = decode_input(&tx.input);
                TransactionData {
                    hash: tx.hash,
                    from: tx.from,
                    to: tx.to,
                    value: tx.value,
                    input: hex::encode(&tx.input),
                    decoded_input,
                }
            })
            .collect();

        let block_data = BlockData {
            number: block.number.unwrap(),
            block: block_json,
            transactions,
        };

        let db = client.database("eth");
        let blocks_collection = db.collection::<BlockData>("blocks");
        blocks_collection.insert_one(block_data, None).await?;

        println!("Stored block: {}", block_number);
    } else {
        println!("Block {} not found", block_number);
    }

    Ok(())
}

fn decode_input(input: &[u8]) -> Option<serde_json::Value> {
    Some(serde_json::json!({
        "input": hex::encode(input)
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        println!("Usage: {} <start_block> <end_block>", args[0]);
        std::process::exit(1);
    }

    let start_block: u64 = args[0].parse().unwrap();
    let end_block: u64 = args[1].parse().unwrap();

    let provider = Provider::<Http>::try_from("https://rpc-mainnet.guapcoinx.com/")?;
    let provider = Arc::new(provider);

    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let client = Arc::new(client);

    let semaphore = Arc::new(Semaphore::new(10));

    let mut tasks = vec![];

    for block_number in start_block..=end_block {
        let client_clone = Arc::clone(&client);
        let provider_clone = Arc::clone(&provider);
        let semaphore_clone = Arc::clone(&semaphore);

        let task = task::spawn(fetch_and_store_block(
            client_clone,
            provider_clone,
            block_number.into(),
            semaphore_clone,
        ));

        tasks.push(task);
    }

    for task in tasks {
        task.await??;
    }

    Ok(())
}
