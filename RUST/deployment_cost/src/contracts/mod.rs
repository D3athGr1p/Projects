use dotenv::dotenv;
use ethers::contract::Contract;
use ethers::prelude::*;
use std::error::Error;

struct Contracts {
    tx: TxHash,
    provider: Provider<Http>,
}

impl Contracts {
    fn new(tx: TxHash, provider: Provider<Http>) -> Result<Contracts, Box<dyn Error>> {
        Ok(Contracts { tx, provider })
    }

    async fn fetch_tx(&self) -> Result<Option<TransactionReceipt>, Box<dyn Error>> {
        let txData = self.provider.get_transaction_receipt(self.tx).await?;
        Ok(txData)
    }
}

fn read_input(msg: &str) -> Option<String> {
    let mut input = String::new();
    println!("{}", msg);
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read address");
    if input.trim().is_empty() {
        return None;
    }

    Some(input.trim().to_string())
}

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let rpc = read_input("Enter Deployed RPC Endpoint")
        .unwrap_or_else(|| "https://data-seed-prebsc-1-s2.binance.org:8545/".to_string());

    let destination = read_input("Enter destination RPC Endpoint")
        .unwrap_or_else(|| "https://bsc-dataseed1.binance.org/".to_string());

    let provider = Provider::<Http>::try_from(rpc)?;

    let txhash = read_input("Enter deployment contract transaction hash").unwrap();

    let txhash: TxHash = txhash.parse()?;

    let contracts = Contracts::new(txhash, provider)?;

    let data = contracts.fetch_tx().await?.unwrap();

    // let pretty_data = serde_json::to_string_pretty(&data)?;

    // println!("{}", pretty_data);

    let used_gas = data.gas_used.unwrap();

    Ok(())
}
