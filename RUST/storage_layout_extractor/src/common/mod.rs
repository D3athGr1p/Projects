use std::{fs::File, io::Read};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use storage_layout_extractor as sle;
use storage_layout_extractor::{
    extractor::{
        chain::{
            version::{ChainVersion, EthereumVersion},
            Chain,
        },
        contract::Contract,
        InitialExtractor,
    },
    tc, vm,
    watchdog::{DynWatchdog, LazyWatchdog},
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledContract {
    deployed_bytecode: DeployedBytecode,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployedBytecode {
    object: String,
}

#[allow(unused)]
pub fn new_extractor_from_bytecode(
    code: impl Into<String>,
    watchdog: DynWatchdog,
) -> anyhow::Result<InitialExtractor> {
    let bytecode = get_bytecode_from_string(code)?;

    let contract = Contract::new(
        bytecode,
        Chain::Ethereum {
            version: EthereumVersion::latest(),
        },
    );

    let vm_config = vm::Config::default();
    let unifier_config = tc::Config::default();

    Ok(sle::new(contract, vm_config, unifier_config, watchdog))
}

#[allow(unused)]
pub fn new_extractor_from_path(path: impl Into<String>) -> anyhow::Result<InitialExtractor> {
    let contract = new_contract_from_file(
        path,
        Chain::Ethereum {
            version: EthereumVersion::latest(),
        },
    )?;
    let vm_config = vm::Config::default();
    let unifier_config = tc::Config::default();

    Ok(sle::new(
        contract,
        vm_config,
        unifier_config,
        LazyWatchdog.in_rc(),
    ))
}

#[allow(unused)]
pub fn new_contract_from_file(path: impl Into<String>, chain: Chain) -> anyhow::Result<Contract> {
    let path = path.into();
    let mut file = File::open(path).map_err(|_| anyhow!("File not available"))?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)
        .map_err(|_| anyhow!("File could not be read"))?;

    let contract_rep: CompiledContract = serde_json::from_slice(contents.as_slice())
        .map_err(|_| anyhow!("Could not parse compiled contract."))?;

    let bytecode = get_bytecode_from_string(contract_rep.deployed_bytecode.object)?;

    Ok(Contract::new(bytecode, chain))
}

pub fn get_bytecode_from_string(code: impl Into<String>) -> anyhow::Result<Vec<u8>> {
    let bytecode_string = code.into();
    // Remove the 0x if it is present
    let no_0x_prefix = match bytecode_string.strip_prefix("0x") {
        Some(no_0x_prefix) => no_0x_prefix,
        None => &bytecode_string,
    };

    let bytecode = hex::decode(no_0x_prefix).map_err(|_| anyhow!("Could not decode hex"))?;
    Ok(bytecode)
}
