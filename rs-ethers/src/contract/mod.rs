use abi::Abi;
use ethers::contract::Contract;
use ethers::{prelude::*, solc::Solc};
use eyre::{ErrReport, Ok, Result};
use std::fs;
use std::sync::Arc;

const RPC: &str = "https://eth.llamarpc.com";

pub struct Contracts {
    abi: Abi,
    address: Address,
    contract: Contract<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl Contracts {
    pub fn new(
        provider: SignerMiddleware<Provider<Http>, LocalWallet>,
        contract_address: &str,
        file_path: &str,
    ) -> Result<Option<Contracts>, ErrReport> {
        let provider = Arc::new(provider.clone());
        let abi_json = fs::read_to_string(file_path)?;
        let abi: Abi = serde_json::from_str(&abi_json)?;

        let contract_address: Address = contract_address.parse()?;

        let contract: Contract<SignerMiddleware<Provider<Http>, LocalWallet>> =
            Contract::new(contract_address, abi.clone(), provider);

        Ok(Some(Self {
            abi,
            address: contract_address,
            contract,
        }))
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

    pub fn print_all_contract_functions(&self) {
        for function in self.abi.functions() {
            println!("Function name: {}", function.name);
            println!("Inputs:");
            for input in &function.inputs {
                println!("  - {}: {}", input.name, input.kind);
            }
            println!("Outputs:");
            for output in &function.outputs {
                println!("  - {}: {}", output.name, output.kind);
            }
            println!();
        }
    }

    pub fn print_all_contract_events(&self) {
        for event in self.abi.events() {
            println!("Event name: {}", event.name);
            println!("Inputs:");
            for evt in &event.inputs {
                println!("  - {}: {}", evt.name, evt.kind);
                println!("      indexed - {}", evt.indexed);
            }
        }
    }
}
