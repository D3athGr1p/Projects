extern crate bitcoincore_rpc;

use std::error::Error;
pub mod client;
pub mod helper;

use client::clients;

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = clients::run() {
        eprintln!("Error occurred: {}", e);
    }

    Ok(())
}
