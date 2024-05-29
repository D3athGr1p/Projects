use ethers::{types::H160, utils::parse_ether};

mod client;
mod contract;

use client::EtherClient;
use dotenv::dotenv;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();

    let _: String = std::env::var("SIGNER_PRIVET_KEY").expect("Failed to read signer privet key");

    let test = EtherClient::new().unwrap();

    let to_adr: H160 = "0x000000000000000000000000000000000000dead"
        .parse()
        .unwrap();

    let val = parse_ether(1u64).unwrap();

    // let _ = test.get_block(10).await;
    let _ = test.create_and_send_tx(to_adr, val).await;
}
