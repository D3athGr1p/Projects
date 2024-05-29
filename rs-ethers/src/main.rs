use ethers::{
    signers::LocalWallet,
    utils::{Anvil, AnvilInstance},
};

mod client;
mod contract;
mod utils;

use client::EtherClient;
use contract::Contracts;
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

    // let to_adr = "0x000000000000000000000000000000000000dead";

    // let value = 1u64;

    // let raw_tx = client.create_raw_coin_tx(to_adr, value, "ether").unwrap();

    // let result = client.send_raw_tx(raw_tx).await;

    // match result {
    //     Ok(_) => {
    //         println!("Transaction executed successfully");
    //     }
    //     Err(err) => {
    //         println!("Error: {:?}", err);
    //     }
    // };

    // let provider = client.get_client().unwrap();

    // let contract = Contracts::new(
    //     provider,
    //     "0xd38D26954C4a8087358b8D698fE5e3255C5Aecea",
    //     "src/contract/IERC20.json",
    // )
    // .unwrap()
    // .unwrap();

    // contract.print_all_contract_events();

    Ok(())
}
