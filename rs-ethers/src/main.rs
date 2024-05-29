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
        println!("Press 2 for connet to the manual RPC");

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

    let wallet: LocalWallet = client.load_wallet(Some(&anvil), key).unwrap();

    // let address = wallet.address();

    // println!("{:?}", client.get_account_balance(address).await?);

    client.set_client_with_privet_key(wallet)?;

    let to_adr: H160 = "0x000000000000000000000000000000000000dead"
        .parse()
        .unwrap();

    let val = parse_ether(1u64).unwrap();

    let _ = client.create_and_send_tx(to_adr, val).await;

    Ok(())
}
