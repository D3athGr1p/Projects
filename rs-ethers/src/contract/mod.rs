use abi::Abi;
use ethers::contract::Contract;
use ethers::{prelude::*, solc::Solc, utils::Anvil};
use eyre::{ErrReport, Ok, Result};
use std::fs;
use std::sync::Arc;
use std::time::Duration;

use crate::client;

const RPC: &str = "https://eth.llamarpc.com";

pub struct Contracts {
    contract: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
}

impl Contracts {
    pub fn new(contract_address: &str, file_path: &str) -> Result<Option<Contracts>, ErrReport> {
        let provider = Arc::new(Provider::<Http>::try_from(RPC)?);
        let abi_json = fs::read_to_string(file_path)?;
        let abi: Abi = serde_json::from_str(&abi_json)?;

        let contract_address: Address = contract_address.parse()?;

        let contract: ContractInstance<Arc<Provider<Http>>, Provider<Http>> =
            Contract::new(contract_address, abi, provider.clone());

        Ok(Some(Self { contract }))
    }

    pub fn compile_contract(
        contract_name: &str,
        sol_file_path: &str,
        output_file_path: &str,
    ) -> Result<(), ErrReport> {
        println!("Generating bindings for {contract_name}\n");

        let abi = if sol_file_path.ends_with(".sol") {
            let contract = Solc::default().compile_source(&sol_file_path)?;
            let abi = contract
                .get(sol_file_path, contract_name)
                .unwrap()
                .abi
                .unwrap();
            Some(serde_json::to_string(abi).unwrap())
        } else {
            None
        };

        let binding = Abigen::new(contract_name, abi.unwrap())?.generate()?;

        let output_file_path = output_file_path.to_owned();

        match output_file_path.is_empty() {
            false => binding.write_to_file(output_file_path)?,
            true => binding.write(&mut std::io::stdout())?,
        };

        Ok(())
    }
}
