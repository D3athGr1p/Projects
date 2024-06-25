extern crate bitcoincore_rpc;

use std::error::Error;
mod client;

use client::run;

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = run() {
        eprintln!("Error occurred: {}", e);
    }

    Ok(())
}
