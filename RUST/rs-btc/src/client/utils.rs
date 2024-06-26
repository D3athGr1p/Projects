use bitcoincore_rpc::bitcoin::{hashes::Hash, hex::FromHex, Address, Network, PubkeyHash};
use serde::Serialize;

fn take_input(message: &str) -> String {
    println!("Please enter {message} : ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_block_number() -> u64 {
    take_input("block_number").parse().unwrap()
}

pub fn get_address() -> Address {
    let address = take_input("address");
    let address_hash: PubkeyHash =
        PubkeyHash::from_slice(&Vec::from_hex(&address).expect("Invalid address hash"))
            .expect("Invalid address hash");

    Address::p2pkh(address_hash, Network::Bitcoin)
}

pub fn get_node_address() -> String {
    take_input("node_address")
}

pub fn prettify_data<T: Serialize>(data: T) {
    let data = serde_json::to_string_pretty(&data).unwrap();
    println!("{}", data);
}

pub fn print_object<T: Serialize>(data: Vec<T>) {
    for item in data.iter() {
        prettify_data(data);
    }
}

pub fn get_bool(message: &str) -> bool {
    let mut result;
    loop {
        let input = take_input(message).parse().unwrap();
        result = match input {
            0 => false,
            1 => true,
            _ => {
                println!("Invalid input");
                continue;
            }
        };
        break;
    }

    result
}

pub fn get_String_array(message: &str) -> Vec<String> {
    let mut vecs = Vec::new();
    let count = take_input(message).parse().unwrap();
    let mut i = 0;
    while i < count {
        let input = take_input(&format!("input {i}"));
        vecs.push(input);
        i += 1;
    }

    vecs
}
