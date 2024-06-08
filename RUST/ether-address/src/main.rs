use rand::rngs::OsRng;
use rand::RngCore;
use rayon::prelude::*;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn generate_address(secret_key: &SecretKey) -> String {
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, secret_key);
    let public_key_serialized = public_key.serialize_uncompressed();

    let mut hasher = Keccak256::new();
    hasher.update(&public_key_serialized[1..]);
    let result = hasher.finalize();
    format!("0x{}", hex::encode(&result[12..]))
}

fn generate_key_pair() -> (SecretKey, String) {
    let mut rng = OsRng;
    let mut secret_key_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_key_bytes);
    let secret_key =
        SecretKey::from_slice(&secret_key_bytes).expect("32 bytes, within curve order");
    let address = generate_address(&secret_key);
    (secret_key, address)
}

fn main() {
    let target_address = "0xabdabcabcabcabcabcabcabcabcabcabcabcabca".to_string();
    let found_address = Arc::new(Mutex::new(None));
    let num_threads = num_cpus::get();

    let start_time = Instant::now();
    (0..num_threads).into_par_iter().for_each(|_| loop {
        let (secret_key, address) = generate_key_pair();
        if address == target_address {
            let mut found = found_address.lock().unwrap();
            if found.is_none() {
                *found = Some(secret_key);
            }
            break;
        }
    });

    if let Some(secret_key) = found_address.lock().unwrap().as_ref() {
        println!("Found matching address! Secret Key: {:?}", secret_key);
    } else {
        println!("No matching address found.");
    }

    println!("Execution time: {:?}", start_time.elapsed());
}
