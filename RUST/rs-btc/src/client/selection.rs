use std::collections::HashMap;

const KEYS: [&str; 8] = [
    "block",
    "blockchain",
    "transaction",
    "wallet",
    "network",
    "utils",
    "psbt",
    "mempool",
];

pub struct Selector<'a> {
    keys: [&'a str; 8],
    function_selector: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Selector<'a> {
    pub fn new() -> Selector<'a> {
        Selector {
            keys: KEYS,
            function_selector: HashMap::new(),
        }
    }

    pub fn insert_block_kv(&mut self) {
        let functions = vec![
            "get_block_count:r:m",
            "get_latest_block_hash:r:m",
            "get_block:r:m",
            "get_best_block:r:m",
            "get_block_hex:r",
            "get_block_info:r",
            "get_block_header:r",
            "get_best_block_hash:r:m",
            "get_block_hash:r",
            "get_block_stats:r",
            "get_block_header_info:r",
            "get_block_template:r:m",
            "get_block_stats_fields:r",
            "get_block_filter:r",
            "get_chain_tips:r",
            "invalidate_block:w",
            "reconsider_block:w",
            "wait_for_new_block:w",
            "submit_block:w:m",
            "submit_block_bytes:w",
            "submit_block_hex:w:m",
            "list_since_block:w",
            "generate:w:m",
            "generate_to_address:w:m",
        ];

        self.function_selector.insert("block", functions);
    }

    pub fn insert_blockchain_kv(&mut self) {
        let functions = vec!["rescan_blockchain:w", "scan_tx_out_set_blocking:W"];

        self.function_selector.insert("blockchain", functions);
    }

    pub fn insert_transaction_kv(&mut self) {
        let functions = vec![
            "get_raw_transaction_info:r:m",
            "get_tx_out_proof:r",
            "get_tx_out_set_info:r",
            "decode_raw_transaction:r:m",
            "send_raw_transaction:w:m",
            "send_to_address:w:m",
            "create_raw_transaction_hex:w:m",
            "fund_raw_transaction:w",
            "sign_raw_transaction:w:d",
            "sign_raw_transaction_with_wallet:w",
            "sign_raw_transaction_with_key:w",
            "combine_raw_transaction:w",
            "create_raw_transaction:w:m",
        ];

        self.function_selector.insert("transaction", functions);
    }

    pub fn insert_wallet_kv(&mut self) {
        let functions = vec![
            "list_wallets:r",
            "list_wallet_dir:r",
            "get_wallet_info:r",
            "dump_private_key:r",
            "get_balances:r:m",
            "get_received_by_address:r",
            "list_unspent:r:m",
            "list_received_by_address:r",
            "get_new_address:r",
            "get_raw_change_address:r",
            "get_address_info:r",
            "load_wallet:w",
            "backup_wallet:w",
            "encrypt_wallet:w",
            "set_label:w",
            "lock_unspent:w",
            "unlock_unspent:w",
            "backup_wallet:w",
            "unlock_unspent_all:w",
        ];

        self.function_selector.insert("wallet", functions);
    }

    pub fn insert_network_kv(&mut self) {
        let functions = vec![
            "get_network_info:r",
            "version:r:m",
            "get_difficulty:r:m",
            "get_connection_count:r",
            "get_mining_info:r:m",
            "get_blockchain_info:r:m",
            "get_added_node_info:r",
            "list_banned:r",
            "get_peer_info:r",
            "ping:r",
            "get_node_addresses:r",
            "add_node:w",
            "remove_node:w",
            "onetry_node:w",
            "disconnect_node:w",
            "disconnect_node_by_id:w",
            "clear_banned:w:m",
            "add_ban:w",
            "remove_ban:w",
            "set_network_active:w",
        ];

        self.function_selector.insert("network", functions);
    }

    pub fn insert_utils_kv(&mut self) {
        let functions = vec![
            "stop:w:m",
            "verify_message:r",
            "derive_addresses:r",
            "get_descriptor_info:r",
            "get_index_info:r",
            "estimate_smart_fee:r",
            "key_pool_refill:w",
        ];

        self.function_selector.insert("utils", functions);
    }

    pub fn insert_psbt_kv(&mut self) {
        let functions = vec![
            "create_psbt:w",
            "join_psbt:r",
            "combine_psbt:w",
            "finalize_psbt:w",
            "wallet_create_funded_psbt:w",
        ];

        self.function_selector.insert("psbt", functions);
    }

    pub fn insert_mempool_kv(&mut self) {
        let functions = vec![
            "test_mempool_accept:r",
            "get_mempool_entry:r",
            "get_raw_mempool:r:m",
            "get_raw_mempool_verbose:r",
        ];

        self.function_selector.insert("mempool", functions);
    }

    pub fn get_keys(&self) -> [&str; 8] {
        self.keys
    }

    pub fn get_values(&self, key: &str) -> Option<&Vec<&str>> {
        self.function_selector.get(key)
    }

    pub fn print_with_key(&self, key: &str) {
        for values in &self.function_selector.get(key.trim()) {
            for item in values.iter() {
                let value: Vec<&str> = item.split(":").collect();
                let (function, f_type, m_used) = match value {
                    value if value.len() == 2 => {
                        if value[1] == "r" {
                            (value[0], "Read", "")
                        } else {
                            (value[0], "Write", "")
                        }
                    }
                    value if value.len() == 3 => {
                        let op;
                        if value[1] == "r" {
                            op = "Read";
                        } else {
                            op = "Write";
                        }
                        let ty;
                        if value[2] == "m" {
                            ty = "Most Used";
                        } else {
                            ty = "Deprecated";
                        }

                        (value[0], op, ty)
                    }
                    _ => unreachable!(),
                };

                println!("    {f_type} {function} [{m_used}]");
            }
        }
    }

    pub fn print_all(&self) {
        for (key, value) in &self.function_selector {
            println!("{key}: ");
            for item in value {
                println!("    {item}");
            }
        }
    }

    pub fn print_all_keys(&self) {
        for (key, _) in &self.function_selector {
            println!("      {key} ");
        }
    }
}
