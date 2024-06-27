use bitcoincore_rpc::bitcoin::{hashes::Hash, hex::FromHex, Address, Network, PubkeyHash, Txid};
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
        prettify_data(item);
    }
}

pub fn get_bool(message: &str) -> bool {
    let result;
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

pub fn get_string_array(message: &str) -> Vec<String> {
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

pub fn get_txid_array(message: &str) -> Vec<Txid> {
    let mut vecs = Vec::new();
    let count = take_input(message).parse().unwrap();
    let mut i = 0;
    while i < count {
        let input = take_input(&format!("input {i}")).parse().unwrap();
        vecs.push(input);
        i += 1;
    }

    vecs
}

pub fn print_hashmap<T: IntoIterator<Item = (U, Z)>, U: std::fmt::Debug, Z: std::fmt::Debug>(
    map: T,
) {
    for (key, value) in map {
        println!("{:#?} : {:#?}", key, value);
    }
}
