pub fn print_block_functions() {
    println!("\nRead Functions");
    println!("      get_block_count");
    println!("      get_latest_block_hash");
    println!("      get_block");
    println!("      get_best_block");
    println!("      get_block_hex");
    println!("      get_block_info");
    println!("      get_block_header");
    println!("      get_best_block_hash");
    println!("      get_block_hash");
    println!("      get_block_stats");
    println!("      get_block_header_info");
    println!("      get_block_template");
    println!("      get_block_stats_fields");
    println!("      get_block_filter");
    println!("      get_chain_tips");
    println!("\nWrite Functions");
    println!("      invalidate_block");
    println!("      reconsider_block");
    println!("      wait_for_new_block");
    println!("      submit_block");
    println!("      submit_block_bytes");
    println!("      submit_block_hex");
    println!("      list_since_block");
    println!("      generate");
    println!("      generate_to_address");
    println!("\nMost Used Functions");
    println!("      get_block_count");
    println!("      get_latest_block_hash");
    println!("      get_block");
    println!("      get_best_block");
    println!("      get_block_template");
    println!("      submit_block");
    println!("      generate");
    println!("      generate_to_address");
}

pub fn print_blockchain_function() {
    println!("\nWrite Functions");
    println!("      rescan_blockchain");
    println!("      scan_tx_out_set_blocking");
}

pub fn print_tx_functions() {
    println!("\nRead Functions");
    println!("      get_raw_transaction_info");
    println!("      get_tx_out_proof");
    println!("      get_tx_out_set_info");
    println!("      decode_raw_transaction");
    println!("\nWrite Functions");
    println!("      send_raw_transaction");
    println!("      send_to_address");
    println!("      create_raw_transaction_hex");
    println!("      fund_raw_transaction");
    println!("      sign_raw_transaction [deprecated]");
    println!("      sign_raw_transaction_with_wallet");
    println!("      sign_raw_transaction_with_key");
    println!("      combine_raw_transaction");
    println!("      create_raw_transaction");
    println!("\nMost Used Functions");
    println!("      create_raw_transaction");
    println!("      decode_raw_transaction");
    println!("      send_raw_transaction");
    println!("      send_to_address");
}

pub fn print_utils_functions() {
    println!("\nRead Functions");
    println!("      stop");
    println!("      verify_message");
    println!("      derive_addresses");
    println!("      get_descriptor_info");
    println!("      get_index_info");
    println!("      estimate_smart_fee");
    println!("\nWrite Functions");
    println!("      key_pool_refill");
    println!("\nMost Used Functions");
    println!("      stop");
}

pub fn print_wallet_functions() {
    println!("\nRead Functions");
    println!("      list_wallets");
    println!("      list_wallet_dir");
    println!("      get_wallet_info");
    println!("      dump_private_key");
    println!("      get_balances");
    println!("      get_received_by_address");
    println!("      list_unspent");
    println!("      list_received_by_address");
    println!("      get_new_address");
    println!("      get_raw_change_address");
    println!("      get_address_info");
    println!("\nWrite Functions");
    println!("      load_wallet");
    println!("      backup_wallet");
    println!("      encrypt_wallet");
    println!("      set_label");
    println!("      lock_unspent");
    println!("      unlock_unspent");
    println!("      unlock_unspent_all");
    println!("\nMost Used Functions");
    println!("      get_wallet_info");
    println!("      list_wallets");
    println!("      get_balances");
    println!("      get_address_info");
    println!("      load_wallet");
}

pub fn print_network_functions() {
    println!("\nRead Functions");
    println!("      get_network_info");
    println!("      version");
    println!("      get_difficulty");
    println!("      get_connection_count");
    println!("      get_mining_info");
    println!("      get_blockchain_info");
    println!("      get_added_node_info");
    println!("      list_banned");
    println!("      get_peer_info");
    println!("      ping");
    println!("      get_node_addresses");
    println!("\nWrite Functions");
    println!("      add_node");
    println!("      remove_node");
    println!("      onetry_node");
    println!("      disconnect_node");
    println!("      disconnect_node_by_id");
    println!("      clear_banned");
    println!("      add_ban");
    println!("      remove_ban");
    println!("      set_network_active");
    println!("\nMost Used Functions");
    println!("      version");
    println!("      get_difficulty");
    println!("      get_network_info");
    println!("      get_blockchain_info");
    println!("      clear_banned");
}

pub fn print_psbt_functions() {
    println!("\nWrite Functions");
    println!("      create_psbt");
    println!("      join_psbt");
    println!("      combine_psbt");
    println!("      finalize_psbt");
    println!("      wallet_create_funded_psbt");
}

pub fn print_mempool_functions() {
    println!("\nRead Functions");
    println!("      test_mempool_accept");
    println!("      get_mempool_entry");
    println!("      get_raw_mempool");
    println!("      get_raw_mempool_verbose");
    println!("\nMost Used Functions");
    println!("      get_raw_mempool");
}

pub fn print_help_message() {
    println!("\n\nPress 1 help for block");
    println!("Press 2 help for blockchain");
    println!("Press 3 help for transaction");
    println!("Press 4 help for wallet");
    println!("Press 5 help for network");
    println!("Press 6 help for utils");
    println!("Press 7 help for psbt");
    println!("Press 8 help for mempool");
    println!("Press 9 for exit\n\n");
}

pub fn print_help(input: u8) {
    match input {
        1 => print_block_functions(),
        2 => print_blockchain_function(),
        3 => print_tx_functions(),
        4 => print_wallet_functions(),
        5 => print_network_functions(),
        6 => print_utils_functions(),
        7 => print_psbt_functions(),
        8 => print_mempool_functions(),
        _ => unreachable!(),
    }
}
