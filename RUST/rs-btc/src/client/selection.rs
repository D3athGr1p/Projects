use std::{collections::HashMap, iter};

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
            "get_block_count",
            "get_latest_block_hash",
            "get_block",
            "get_best_block",
            "get_block_hex",
            "get_block_info",
            "get_block_header",
            "get_best_block_hash",
            "get_block_hash",
            "get_block_stats",
            "get_block_header_info",
            "get_block_template",
            "get_block_stats_fields",
            "get_block_filter",
            "get_chain_tips",
            "invalidate_block",
            "reconsider_block",
            "wait_for_new_block",
            "submit_block",
            "submit_block_bytes",
            "submit_block_hex",
            "list_since_block",
            "generate",
            "generate_to_address",
        ];

        self.function_selector.insert("block", functions);
    }

    pub fn insert_blockchain_kv(&mut self) {
        let functions = vec!["rescan_blockchain", "scan_tx_out_set_blocking"];

        self.function_selector.insert("blockchain", functions);
    }

    pub fn insert_transaction_kv(&mut self) {
        let functions = vec![
            "get_raw_transaction_info",
            "get_tx_out_proof",
            "get_tx_out_set_info",
            "decode_raw_transaction",
            "send_raw_transaction",
            "send_to_address",
            "create_raw_transaction_hex",
            "fund_raw_transaction",
            "sign_raw_transaction",
            "sign_raw_transaction_with_wallet",
            "sign_raw_transaction_with_key",
            "combine_raw_transaction",
            "create_raw_transaction",
        ];

        self.function_selector.insert("transaction", functions);
    }

    pub fn insert_wallet_kv(&mut self) {
        let functions = vec![
            "list_wallets",
            "list_wallet_dir",
            "get_wallet_info",
            "dump_private_key",
            "get_balances",
            "get_received_by_address",
            "list_unspent",
            "list_received_by_address",
            "get_new_address",
            "get_raw_change_address",
            "get_address_info",
            "load_wallet",
            "backup_wallet",
            "encrypt_wallet",
            "set_label",
            "lock_unspent",
            "unlock_unspent",
            "backup_wallet",
            "unlock_unspent_all",
        ];

        self.function_selector.insert("wallet", functions);
    }

    pub fn insert_network_kv(&mut self) {
        let functions = vec![
            "get_network_info",
            "version",
            "get_difficulty",
            "get_connection_count",
            "get_mining_info",
            "get_blockchain_info",
            "get_added_node_info",
            "list_banned",
            "get_peer_info",
            "ping",
            "get_node_addresses",
            "add_node",
            "remove_node",
            "onetry_node",
            "disconnect_node",
            "disconnect_node_by_id",
            "clear_banned",
            "add_ban",
            "remove_ban",
            "set_network_active",
        ];

        self.function_selector.insert("network", functions);
    }

    pub fn insert_utils_kv(&mut self) {
        let functions = vec![
            "stop",
            "verify_message",
            "derive_addresses",
            "get_descriptor_info",
            "get_index_info",
            "estimate_smart_fee",
            "key_pool_refill",
        ];

        self.function_selector.insert("utils", functions);
    }

    pub fn insert_psbt_kv(&mut self) {
        let functions = vec![
            "create_psbt",
            "join_psbt",
            "combine_psbt",
            "finalize_psbt",
            "wallet_create_funded_psbt",
        ];

        self.function_selector.insert("psbt", functions);
    }

    pub fn insert_mempool_kv(&mut self) {
        let functions = vec![
            "test_mempool_accept",
            "get_mempool_entry",
            "get_raw_mempool",
            "get_raw_mempool_verbose",
        ];

        self.function_selector.insert("mempool", functions);
    }

    pub fn get_keys(&self) -> [&str; 8] {
        self.keys
    }

    pub fn print(&self) {
        for (book, review) in &self.function_selector {
            println!("{book}: ");
            for item in review {
                println!("    {item}");
            }
        }
    }
}
