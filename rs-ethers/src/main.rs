use ethers::{
    signers::LocalWallet,
    types::H160,
    utils::{parse_ether, Anvil, AnvilInstance},
};

mod client;
mod contract;
mod utils;

use client::EtherClient;
use dotenv::dotenv;
use std::error::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let mut user_input = String::new();
    let mut end_point = String::new();
    let mut key = String::new();
    let anvil: AnvilInstance = Anvil::new().spawn();

    loop {
        println!("Welcome to rs-ether server");
        println!("Press 1 for Anvil to connect");
        println!("Press 2 for connect to the manual RPC");

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        match user_input.trim().as_ref() {
            "1" => {
                end_point = anvil.endpoint().to_owned();
                break;
            }
            "2" => {
                std::io::stdin()
                    .read_line(&mut end_point)
                    .expect("Failed to read end_point");
                end_point = end_point.trim().to_owned();

                println!("Trying to read private key from .env file with variable named: SIGNER_PRIVET_KEY");

                key = std::env::var("SIGNER_PRIVET_KEY").expect("Failed to read signer privet key");
                break;
            }
            _ => {
                println!("Invalid input");
                user_input.clear();
            }
        }
    }
    let mut client = EtherClient::new(end_point).unwrap();

    let chain_id = client.get_chain_id().await.unwrap();

    let wallet: LocalWallet = client.load_wallet(Some(&anvil), key).unwrap();

    client.set_client_with_privet_key(wallet, chain_id.as_u64())?;

    let to_adr: H160 = "0x000000000000000000000000000000000000dead"
        .parse()
        .unwrap();

    let value = parse_ether(1u64).unwrap();

    let value = client.create_and_send_tx(to_adr, value).await;

    match value {
        Ok(_) => {
            println!("Success");
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    };

    Ok(())
}
