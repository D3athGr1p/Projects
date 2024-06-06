use dotenv::dotenv;
use ethers::prelude::*;
use std::error::Error;

struct Contracts {
    tx: TxHash,
    provider: Provider<Http>,
    destination_rpc: Provider<Http>,
}

impl Contracts {
    fn new(
        tx: TxHash,
        provider: Provider<Http>,
        destination_rpc: Provider<Http>,
    ) -> Result<Contracts, Box<dyn Error>> {
        Ok(Contracts {
            tx,
            provider,
            destination_rpc,
        })
    }

    async fn fetch_tx(&self) -> Result<Option<TransactionReceipt>, Box<dyn Error>> {
        let tx_data = self.provider.get_transaction_receipt(self.tx).await?;
        Ok(tx_data)
    }

    async fn fetch_gas_price(&self) -> Result<U256, Box<dyn Error>> {
        let gas_price = self.destination_rpc.get_gas_price().await?;
        Ok(gas_price)
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
    println!();

    Some(input.trim().to_string())
}

fn wei_to_gwei(wei: U256) -> U256 {
    let gwei_divisor = U256::from(1_000_000_000u64);
    wei / gwei_divisor
}

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let rpc = read_input("Enter Deployed RPC Endpoint")
        .unwrap_or_else(|| "https://data-seed-prebsc-1-s2.binance.org:8545/".to_string());

    let destination = read_input("Enter destination RPC Endpoint")
        .unwrap_or_else(|| "https://bsc-dataseed1.binance.org/".to_string());

    let provider = Provider::<Http>::try_from(rpc)?;
    let destination_rpc = Provider::<Http>::try_from(destination)?;

    let txhash = read_input("Enter deployment contract transaction hash").unwrap();

    let txhash: TxHash = txhash.parse()?;

    let contracts = Contracts::new(txhash, provider, destination_rpc)?;

    let data = contracts.fetch_tx().await?.unwrap();

    // let pretty_data = serde_json::to_string_pretty(&data)?;

    // println!("{}", pretty_data);

    let used_gas = data.gas_used.unwrap();

    println!("Gas used during deployment: {}", used_gas);

    let multiplier: u64 = read_input("Enter Deployment Multiplier")
        .unwrap_or_else(|| "2".to_string())
        .parse()?;

    let current_cost = used_gas * multiplier;

    let latest_price = contracts.fetch_gas_price().await?;

    let latest_price: U256 = wei_to_gwei(latest_price);

    println!("Current Destination GWEI is {}", latest_price);

    let gas_multiplier: U256 = read_input("Enter GAS Multiplier")
        .unwrap_or_else(|| "2".to_string())
        .parse()?;

    let current_cost = current_cost * (latest_price * gas_multiplier);

    println!(
        "Current Destination GWEI is {current_cost} \n
        current deployment gas used = {used_gas},
        number of time to deploy this same contract = {multiplier},
        current destination gas price = {latest_price},
        gas multipliter = {gas_multiplier},
        using formula = (used gas * deployment count) * (current gas * gas multiplier) = deployment cost\n
        ({used_gas} * {multiplier}) * ({latest_price} * {gas_multiplier}) = {current_cost}"
    );

    println!();

    Ok(())
}
