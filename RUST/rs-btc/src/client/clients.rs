use crate::client::{
    selection::Selector,
    utils::{get_address, get_block_number, get_bool, get_string_array, prettify_data},
};

use bitcoincore_rpc::{
    bitcoin::{
        address::{NetworkChecked, NetworkUnchecked},
        block::Header,
        secp256k1::ecdsa::Signature,
        Address, Amount, Block, BlockHash, OutPoint, PrivateKey, PublicKey, Script, Transaction,
        Txid,
    },
    json::{
        AddMultiSigAddressResult, AddressType, BlockRef, BlockStatsFields,
        CreateRawTransactionInput, DecodeRawTransactionResult, EstimateMode,
        EstimateSmartFeeResult, FinalizePsbtResult, FundRawTransactionOptions,
        FundRawTransactionResult, GetAddedNodeInfoResult, GetAddressInfoResult, GetBalancesResult,
        GetBlockFilterResult, GetBlockHeaderResult, GetBlockResult, GetBlockStatsResult,
        GetBlockStatsResultPartial, GetBlockTemplateCapabilities, GetBlockTemplateModes,
        GetBlockTemplateResult, GetBlockTemplateRules, GetBlockchainInfoResult, GetChainTipsResult,
        GetDescriptorInfoResult, GetIndexInfoResult, GetMempoolEntryResult, GetMiningInfoResult,
        GetNetTotalsResult, GetNetworkInfoResult, GetNodeAddressesResult, GetPeerInfoResult,
        GetRawTransactionResult, GetTransactionResult, GetTxOutSetInfoResult, GetWalletInfoResult,
        HashOrHeight, ImportDescriptors, ImportMultiOptions, ImportMultiRequest, ImportMultiResult,
        ListBannedResult, ListReceivedByAddressResult, ListSinceBlockResult, ListTransactionResult,
        ListUnspentQueryOptions, ListUnspentResultEntry, LoadWalletResult, PubKeyOrAddress,
        ScanTxOutRequest, ScanTxOutResult, SigHashType, SignRawTransactionInput,
        SignRawTransactionResult, TxOutSetHashType, UnloadWalletResult,
        WalletCreateFundedPsbtOptions, WalletCreateFundedPsbtResult, WalletProcessPsbtResult,
    },
    Auth, Client, RawTx, RpcApi,
};
use std::{collections::HashMap, error::Error};

use super::utils::{get_node_address, get_txid_array, print_hashmap, print_object};

pub struct Clients<'a> {
    rpc: Client,
    selector: Selector<'a>,
}

impl<'a> Clients<'a> {
    fn new(
        rpc_url: &str,
        rpc_user: String,
        rpc_password: String,
    ) -> Result<Clients, Box<dyn Error>> {
        let rpc = Client::new(rpc_url, Auth::UserPass(rpc_user, rpc_password))?;
        let mut selector = Selector::new();

        selector.insert_block_kv();
        selector.insert_blockchain_kv();
        selector.insert_transaction_kv();
        selector.insert_wallet_kv();
        selector.insert_network_kv();
        selector.insert_utils_kv();
        selector.insert_psbt_kv();
        selector.get_keys();

        Ok(Clients { rpc, selector })
    }

    fn get_block_count(&self) -> u64 {
        self.rpc.get_block_count().unwrap()
    }

    fn get_latest_block_hash(&self) -> Result<BlockHash, Box<dyn Error>> {
        let block_hash = self.rpc.get_best_block_hash()?;
        Ok(block_hash)
    }

    fn get_block(&self, block_hash: BlockHash) -> Result<Block, Box<dyn Error>> {
        let block = self.rpc.get_block(&block_hash)?;
        Ok(block)
    }

    fn get_best_block(&self) -> Result<Block, Box<dyn Error>> {
        let block_hash = self.get_latest_block_hash()?;
        let block = self.get_block(block_hash)?;
        Ok(block)
    }

    fn get_network_info(&self) -> Result<GetNetworkInfoResult, Box<dyn Error>> {
        let network_info = self.rpc.get_network_info()?;
        Ok(network_info)
    }

    fn get_index_info(&self) -> Result<GetIndexInfoResult, Box<dyn Error>> {
        let index_info = self.rpc.get_index_info()?;
        Ok(index_info)
    }

    fn version(&self) -> Result<usize, Box<dyn Error>> {
        let version = self.rpc.version()?;
        Ok(version)
    }

    fn load_wallet(&self, wallet: &str) -> Result<LoadWalletResult, Box<dyn Error>> {
        let wallet = self.rpc.load_wallet(wallet)?;
        Ok(wallet)
    }

    fn unload_wallet(&self, wallet: Option<&str>) -> Result<UnloadWalletResult, Box<dyn Error>> {
        let wallet = self.rpc.unload_wallet(wallet)?.unwrap();
        Ok(wallet)
    }

    fn list_wallets(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let wallet_list = self.rpc.list_wallets()?;
        Ok(wallet_list)
    }

    fn list_wallet_dir(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let wallet_list_dir = self.rpc.list_wallet_dir()?;
        Ok(wallet_list_dir)
    }

    fn get_wallet_info(&self) -> Result<GetWalletInfoResult, Box<dyn Error>> {
        let wallet_info = self.rpc.get_wallet_info()?;
        Ok(wallet_info)
    }

    fn backup_wallet(&self, destination_path: &str) -> Result<(), Box<dyn Error>> {
        self.rpc.backup_wallet(Some(destination_path))?;
        Ok(())
    }

    fn dump_private_key(&self, address: Address) -> Result<PrivateKey, Box<dyn Error>> {
        let privet_key = self.rpc.dump_private_key(&address)?;
        Ok(privet_key)
    }

    fn encrypt_wallet(&self, passphrase: &str) -> Result<(), Box<dyn Error>> {
        self.rpc.encrypt_wallet(&passphrase)?;
        Ok(())
    }

    fn get_difficulty(&self) -> Result<f64, Box<dyn Error>> {
        let difficulty = self.rpc.get_difficulty()?;
        Ok(difficulty)
    }

    fn get_connection_count(&self) -> Result<usize, Box<dyn Error>> {
        let difficulty = self.rpc.get_connection_count()?;
        Ok(difficulty)
    }

    fn get_block_hex(&self, hash: BlockHash) -> Result<String, Box<dyn Error>> {
        let hex = self.rpc.get_block_hex(&hash)?;
        Ok(hex)
    }

    fn get_block_info(&self, hash: BlockHash) -> Result<GetBlockResult, Box<dyn Error>> {
        let block_info = self.rpc.get_block_info(&hash)?;
        Ok(block_info)
    }

    fn get_block_header(&self, hash: BlockHash) -> Result<Header, Box<dyn Error>> {
        let block_header = self.rpc.get_block_header(&hash)?;
        Ok(block_header)
    }

    fn get_mining_info(&self) -> Result<GetMiningInfoResult, Box<dyn Error>> {
        let mining_info = self.rpc.get_mining_info()?;
        Ok(mining_info)
    }

    fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResult, Box<dyn Error>> {
        let blockchain_info = self.rpc.get_blockchain_info()?;
        Ok(blockchain_info)
    }

    fn get_best_block_hash(&self) -> Result<BlockHash, Box<dyn Error>> {
        let best_block_hash = self.rpc.get_best_block_hash()?;
        Ok(best_block_hash)
    }

    fn get_block_hash(&self, height: u64) -> Result<BlockHash, Box<dyn Error>> {
        let block_hash = self.rpc.get_block_hash(height)?;
        Ok(block_hash)
    }

    fn get_block_stats(&self, height: u64) -> Result<GetBlockStatsResult, Box<dyn Error>> {
        let block_stats = self.rpc.get_block_stats(height)?;
        Ok(block_stats)
    }

    fn get_balances(&self) -> Result<GetBalancesResult, Box<dyn Error>> {
        let balance = self.rpc.get_balances()?;
        Ok(balance)
    }

    fn get_received_by_address(
        &self,
        address: &Address,
        minconf: Option<u32>,
    ) -> Result<Amount, Box<dyn Error>> {
        let amount = self.rpc.get_received_by_address(address, minconf)?;
        Ok(amount)
    }

    fn set_label(&self, address: &Address, label: &str) -> Result<(), Box<dyn Error>> {
        self.rpc.set_label(address, label)?;
        Ok(())
    }

    fn key_pool_refill(&self, new_size: Option<usize>) -> Result<(), Box<dyn Error>> {
        self.rpc.key_pool_refill(new_size)?;
        Ok(())
    }

    fn lock_unspent(&self, outputs: &[OutPoint]) -> Result<bool, Box<dyn Error>> {
        let result = self.rpc.lock_unspent(outputs)?;
        Ok(result)
    }

    fn unlock_unspent(&self, outputs: &[OutPoint]) -> Result<bool, Box<dyn Error>> {
        let result = self.rpc.unlock_unspent(outputs)?;
        Ok(result)
    }

    fn unlock_unspent_all(&self) -> Result<bool, Box<dyn Error>> {
        let result = self.rpc.unlock_unspent_all()?;
        Ok(result)
    }

    fn stop(&self) -> Result<(), Box<dyn Error>> {
        self.rpc.stop()?;
        Ok(())
    }

    fn get_raw_change_address(
        &self,
        address_type: Option<AddressType>,
    ) -> Result<Address<NetworkUnchecked>, Box<dyn Error>> {
        let address = self.rpc.get_raw_change_address(address_type)?;
        Ok(address)
    }

    fn get_address_info(&self, address: &Address) -> Result<GetAddressInfoResult, Box<dyn Error>> {
        let address_info = self.rpc.get_address_info(address)?;
        Ok(address_info)
    }

    fn generate(
        &self,
        block_num: u64,
        maxtries: Option<u64>,
    ) -> Result<Vec<BlockHash>, Box<dyn Error>> {
        let block_hashes = self.rpc.generate(block_num, maxtries)?;
        Ok(block_hashes)
    }

    fn invalidate_block(&self, block_hash: BlockHash) -> Result<(), Box<dyn Error>> {
        self.rpc.invalidate_block(&block_hash)?;
        Ok(())
    }

    fn reconsider_block(&self, block_hash: BlockHash) -> Result<(), Box<dyn Error>> {
        self.rpc.reconsider_block(&block_hash)?;
        Ok(())
    }

    fn get_mempool_entry(&self, txid: Txid) -> Result<GetMempoolEntryResult, Box<dyn Error>> {
        let entry = self.rpc.get_mempool_entry(&txid)?;
        Ok(entry)
    }

    fn get_raw_mempool(&self) -> Result<Vec<Txid>, Box<dyn Error>> {
        let raw_mempool = self.rpc.get_raw_mempool()?;
        Ok(raw_mempool)
    }

    fn get_chain_tips(&self) -> Result<GetChainTipsResult, Box<dyn Error>> {
        let chain_tips = self.rpc.get_chain_tips()?;
        Ok(chain_tips)
    }

    fn add_node(&self, addr: &str) -> Result<(), Box<dyn Error>> {
        self.rpc.add_node(addr)?;
        Ok(())
    }

    fn remove_node(&self, addr: &str) -> Result<(), Box<dyn Error>> {
        self.rpc.remove_node(addr)?;
        Ok(())
    }

    fn onetry_node(&self, addr: &str) -> Result<(), Box<dyn Error>> {
        self.rpc.onetry_node(addr)?;
        Ok(())
    }

    fn disconnect_node(&self, addr: &str) -> Result<(), Box<dyn Error>> {
        self.rpc.disconnect_node(addr)?;
        Ok(())
    }

    fn disconnect_node_by_id(&self, node_id: u32) -> Result<(), Box<dyn Error>> {
        self.rpc.disconnect_node_by_id(node_id)?;
        Ok(())
    }

    fn get_added_node_info(
        &self,
        node: Option<&str>,
    ) -> Result<Vec<GetAddedNodeInfoResult>, Box<dyn Error>> {
        let node_info = self.rpc.get_added_node_info(node)?;
        Ok(node_info)
    }

    fn list_banned(&self) -> Result<Vec<ListBannedResult>, Box<dyn Error>> {
        let banned = self.rpc.list_banned()?;
        Ok(banned)
    }

    fn clear_banned(&self) -> Result<(), Box<dyn Error>> {
        self.rpc.clear_banned()?;
        Ok(())
    }

    fn add_ban(&self, subnet: &str, bantime: u64, absolute: bool) -> Result<(), Box<dyn Error>> {
        self.rpc.add_ban(subnet, bantime, absolute)?;
        Ok(())
    }

    fn remove_ban(&self, subnet: &str) -> Result<(), Box<dyn Error>> {
        self.rpc.remove_ban(subnet)?;
        Ok(())
    }

    fn set_network_active(&self, state: bool) -> Result<bool, Box<dyn Error>> {
        let success = self.rpc.set_network_active(state)?;
        Ok(success)
    }

    fn get_peer_info(&self) -> Result<Vec<GetPeerInfoResult>, Box<dyn Error>> {
        let peer_info = self.rpc.get_peer_info()?;
        Ok(peer_info)
    }

    fn ping(&self) -> Result<(), Box<dyn Error>> {
        self.rpc.ping()?;
        Ok(())
    }

    fn send_raw_transaction<R: RawTx>(&self, tx: R) -> Result<Txid, Box<dyn Error>> {
        let tx_id = self.rpc.send_raw_transaction(tx)?;
        Ok(tx_id)
    }

    fn wait_for_new_block(
        &self,
        blockhash: BlockHash,
        timeout: u64,
    ) -> Result<BlockRef, Box<dyn Error>> {
        let block_ref = self.rpc.wait_for_block(&blockhash, timeout)?;
        Ok(block_ref)
    }

    fn get_descriptor_info(&self, desc: &str) -> Result<GetDescriptorInfoResult, Box<dyn Error>> {
        let descriptor_info = self.rpc.get_descriptor_info(desc)?;
        Ok(descriptor_info)
    }

    fn join_psbt(&self, psbts: &[String]) -> Result<String, Box<dyn Error>> {
        let psbts = self.rpc.join_psbt(psbts)?;
        Ok(psbts)
    }

    fn combine_psbt(&self, psbts: &[String]) -> Result<String, Box<dyn Error>> {
        let psbts = self.rpc.join_psbt(psbts)?;
        Ok(psbts)
    }

    fn combine_raw_transaction(&self, hex_strings: &[String]) -> Result<String, Box<dyn Error>> {
        let hex = self.rpc.join_psbt(hex_strings)?;
        Ok(hex)
    }

    fn finalize_psbt(
        &self,
        psbt: &str,
        extract: Option<bool>,
    ) -> Result<FinalizePsbtResult, Box<dyn Error>> {
        let psbt = self.rpc.finalize_psbt(psbt, extract)?;
        Ok(psbt)
    }

    fn derive_addresses(
        &self,
        descriptor: &str,
        range: Option<[u32; 2]>,
    ) -> Result<Vec<Address<NetworkUnchecked>>, Box<dyn Error>> {
        let addresses = self.rpc.derive_addresses(descriptor, range)?;
        Ok(addresses)
    }

    fn get_net_totals(&self) -> Result<GetNetTotalsResult, Box<dyn Error>> {
        let net_total = self.rpc.get_net_totals()?;
        Ok(net_total)
    }

    fn get_network_hash_ps(
        &self,
        nblocks: Option<u64>,
        height: Option<u64>,
    ) -> Result<f64, Box<dyn Error>> {
        let hash = self.rpc.get_network_hash_ps(nblocks, height)?;
        Ok(hash)
    }

    fn uptime(&self) -> Result<u64, Box<dyn Error>> {
        let uptimes = self.rpc.uptime()?;
        Ok(uptimes)
    }

    fn submit_block(&self, block: Block) -> Result<(), Box<dyn Error>> {
        self.rpc.submit_block(&block)?;
        Ok(())
    }

    fn submit_block_bytes(&self, block_bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        self.rpc.submit_block_bytes(block_bytes)?;
        Ok(())
    }

    fn submit_block_hex(&self, block_hex: &str) -> Result<(), Box<dyn Error>> {
        self.rpc.submit_block_hex(block_hex)?;
        Ok(())
    }

    fn add_multisig_address(
        &self,
        nrequired: usize,
        keys: &[PubKeyOrAddress],
        label: Option<&str>,
        address_type: Option<AddressType>,
    ) -> Result<AddMultiSigAddressResult, Box<dyn Error>> {
        let muti_sign = self
            .rpc
            .add_multisig_address(nrequired, keys, label, address_type)?;
        Ok(muti_sign)
    }

    fn create_wallet(
        &self,
        wallet: &str,
        disable_private_keys: Option<bool>,
        blank: Option<bool>,
        passphrase: Option<&str>,
        avoid_reuse: Option<bool>,
    ) -> Result<LoadWalletResult, Box<dyn Error>> {
        let wallet =
            self.rpc
                .create_wallet(wallet, disable_private_keys, blank, passphrase, avoid_reuse)?;
        Ok(wallet)
    }

    fn get_block_header_info(
        &self,
        hash: &BlockHash,
    ) -> Result<GetBlockHeaderResult, Box<dyn Error>> {
        let header_info = self.rpc.get_block_header_info(hash)?;
        Ok(header_info)
    }

    fn get_block_template(
        &self,
        mode: GetBlockTemplateModes,
        rules: &[GetBlockTemplateRules],
        capabilities: &[GetBlockTemplateCapabilities],
    ) -> Result<GetBlockTemplateResult, Box<dyn Error>> {
        let template = self.rpc.get_block_template(mode, rules, capabilities)?;
        Ok(template)
    }

    fn get_block_stats_fields(
        &self,
        height: u64,
        fields: &[BlockStatsFields],
    ) -> Result<GetBlockStatsResultPartial, Box<dyn Error>> {
        let block_stats = self.rpc.get_block_stats_fields(height, fields)?;
        Ok(block_stats)
    }

    fn get_raw_transaction(
        &self,
        txid: &Txid,
        block_hash: Option<&BlockHash>,
    ) -> Result<Transaction, Box<dyn Error>> {
        let raw = self.rpc.get_raw_transaction(txid, block_hash)?;
        Ok(raw)
    }

    fn get_raw_transaction_hex(
        &self,
        txid: &Txid,
        block_hash: Option<&BlockHash>,
    ) -> Result<String, Box<dyn Error>> {
        let raw_hex = self.rpc.get_raw_transaction_hex(txid, block_hash)?;
        Ok(raw_hex)
    }

    fn get_raw_transaction_info(
        &self,
        txid: &Txid,
        block_hash: Option<&BlockHash>,
    ) -> Result<GetRawTransactionResult, Box<dyn Error>> {
        let tx_result = self.rpc.get_raw_transaction_info(txid, block_hash)?;
        Ok(tx_result)
    }

    fn get_block_filter(
        &self,
        block_hash: &BlockHash,
    ) -> Result<GetBlockFilterResult, Box<dyn Error>> {
        let block_filter = self.rpc.get_block_filter(block_hash)?;
        Ok(block_filter)
    }

    fn get_balance(
        &self,
        minconf: Option<usize>,
        include_watchonly: Option<bool>,
    ) -> Result<Amount, Box<dyn Error>> {
        let balance = self.rpc.get_balance(minconf, include_watchonly)?;
        Ok(balance)
    }

    fn get_transaction(
        &self,
        txid: &Txid,
        include_watchonly: Option<bool>,
    ) -> Result<GetTransactionResult, Box<dyn Error>> {
        let tx = self.rpc.get_transaction(txid, include_watchonly)?;
        Ok(tx)
    }

    fn list_transactions(
        &self,
        label: Option<&str>,
        count: Option<usize>,
        skip: Option<usize>,
        include_watchonly: Option<bool>,
    ) -> Result<Vec<ListTransactionResult>, Box<dyn Error>> {
        let list_tx = self
            .rpc
            .list_transactions(label, count, skip, include_watchonly)?;
        Ok(list_tx)
    }

    fn list_since_block(
        &self,
        blockhash: Option<&BlockHash>,
        target_confirmations: Option<usize>,
        include_watchonly: Option<bool>,
        include_removed: Option<bool>,
    ) -> Result<ListSinceBlockResult, Box<dyn Error>> {
        let list_block = self.rpc.list_since_block(
            blockhash,
            target_confirmations,
            include_watchonly,
            include_removed,
        )?;
        Ok(list_block)
    }

    fn get_tx_out_proof(
        &self,
        txids: &[Txid],
        block_hash: Option<&BlockHash>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let tx_out = self.rpc.get_tx_out_proof(txids, block_hash)?;
        Ok(tx_out)
    }

    fn import_public_key(
        &self,
        pubkey: &PublicKey,
        label: Option<&str>,
        rescan: Option<bool>,
    ) -> Result<(), Box<dyn Error>> {
        self.rpc.import_public_key(pubkey, label, rescan)?;
        Ok(())
    }

    fn import_private_key(
        &self,
        privkey: &PrivateKey,
        label: Option<&str>,
        rescan: Option<bool>,
    ) -> Result<(), Box<dyn Error>> {
        self.rpc.import_private_key(privkey, label, rescan)?;
        Ok(())
    }

    fn import_address(
        &self,
        address: &Address,
        label: Option<&str>,
        rescan: Option<bool>,
    ) -> Result<(), Box<dyn Error>> {
        self.rpc.import_address(address, label, rescan)?;
        Ok(())
    }

    fn import_address_script(
        &self,
        script: &Script,
        label: Option<&str>,
        rescan: Option<bool>,
        p2sh: Option<bool>,
    ) -> Result<(), Box<dyn Error>> {
        self.rpc
            .import_address_script(script, label, rescan, p2sh)?;
        Ok(())
    }

    fn import_multi(
        &self,
        requests: &[ImportMultiRequest],
        options: Option<&ImportMultiOptions>,
    ) -> Result<Vec<ImportMultiResult>, Box<dyn Error>> {
        let result = self.rpc.import_multi(requests, options)?;
        Ok(result)
    }

    fn import_descriptors(
        &self,
        req: ImportDescriptors,
    ) -> Result<Vec<ImportMultiResult>, Box<dyn Error>> {
        let desc = self.rpc.import_descriptors(req)?;
        Ok(desc)
    }

    fn list_unspent(
        &self,
        minconf: Option<usize>,
        maxconf: Option<usize>,
        addresses: Option<&[&Address<NetworkChecked>]>,
        include_unsafe: Option<bool>,
        query_options: Option<ListUnspentQueryOptions>,
    ) -> Result<Vec<ListUnspentResultEntry>, Box<dyn Error>> {
        let list =
            self.rpc
                .list_unspent(minconf, maxconf, addresses, include_unsafe, query_options)?;
        Ok(list)
    }

    fn list_received_by_address(
        &self,
        address_filter: Option<&Address>,
        minconf: Option<u32>,
        include_empty: Option<bool>,
        include_watchonly: Option<bool>,
    ) -> Result<Vec<ListReceivedByAddressResult>, Box<dyn Error>> {
        let list = self.rpc.list_received_by_address(
            address_filter,
            minconf,
            include_empty,
            include_watchonly,
        )?;
        Ok(list)
    }

    fn create_psbt(
        &self,
        inputs: &[CreateRawTransactionInput],
        outputs: &HashMap<String, Amount>,
        locktime: Option<i64>,
        replaceable: Option<bool>,
    ) -> Result<String, Box<dyn Error>> {
        let psbt = self
            .rpc
            .create_psbt(inputs, outputs, locktime, replaceable)?;
        Ok(psbt)
    }

    fn create_raw_transaction_hex(
        &self,
        utxos: &[CreateRawTransactionInput],
        outs: &HashMap<String, Amount>,
        locktime: Option<i64>,
        replaceable: Option<bool>,
    ) -> Result<String, Box<dyn Error>> {
        let raw_hex = self
            .rpc
            .create_raw_transaction_hex(utxos, outs, locktime, replaceable)?;
        Ok(raw_hex)
    }

    fn create_raw_transaction(
        &self,
        utxos: &[CreateRawTransactionInput],
        outs: &HashMap<String, Amount>,
        locktime: Option<i64>,
        replaceable: Option<bool>,
    ) -> Result<Transaction, Box<dyn Error>> {
        let raw = self
            .rpc
            .create_raw_transaction(utxos, outs, locktime, replaceable)?;
        Ok(raw)
    }

    fn decode_raw_transaction<R: RawTx>(
        &self,
        tx: R,
        is_witness: Option<bool>,
    ) -> Result<DecodeRawTransactionResult, Box<dyn Error>> {
        let tx = self.rpc.decode_raw_transaction(tx, is_witness)?;
        Ok(tx)
    }

    fn fund_raw_transaction<R: RawTx>(
        &self,
        tx: R,
        options: Option<&FundRawTransactionOptions>,
        is_witness: Option<bool>,
    ) -> Result<FundRawTransactionResult, Box<dyn Error>> {
        let raw = self.rpc.fund_raw_transaction(tx, options, is_witness)?;
        Ok(raw)
    }

    fn sign_raw_transaction<R: RawTx>(
        &self,
        tx: R,
        utxos: Option<&[SignRawTransactionInput]>,
        private_keys: Option<&[PrivateKey]>,
        sighash_type: Option<SigHashType>,
    ) -> Result<SignRawTransactionResult, Box<dyn Error>> {
        let signed = self
            .rpc
            .sign_raw_transaction(tx, utxos, private_keys, sighash_type)?;
        Ok(signed)
    }

    fn sign_raw_transaction_with_wallet<R: RawTx>(
        &self,
        tx: R,
        utxos: Option<&[SignRawTransactionInput]>,
        sighash_type: Option<SigHashType>,
    ) -> Result<SignRawTransactionResult, Box<dyn Error>> {
        let signed = self
            .rpc
            .sign_raw_transaction_with_wallet(tx, utxos, sighash_type)?;
        Ok(signed)
    }

    fn sign_raw_transaction_with_key<R: RawTx>(
        &self,
        tx: R,
        privkeys: &[PrivateKey],
        prevtxs: Option<&[SignRawTransactionInput]>,
        sighash_type: Option<SigHashType>,
    ) -> Result<SignRawTransactionResult, Box<dyn Error>> {
        let signed = self
            .rpc
            .sign_raw_transaction_with_key(tx, privkeys, prevtxs, sighash_type)?;
        Ok(signed)
    }

    fn verify_message(
        &self,
        address: &Address,
        signature: &Signature,
        message: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let is_verified = self.rpc.verify_message(address, signature, message)?;
        Ok(is_verified)
    }

    fn get_new_address(
        &self,
        label: Option<&str>,
        address_type: Option<AddressType>,
    ) -> Result<Address<NetworkUnchecked>, Box<dyn Error>> {
        let address = self.rpc.get_new_address(label, address_type)?;
        Ok(address)
    }

    fn generate_to_address(
        &self,
        block_num: u64,
        address: &Address<NetworkChecked>,
    ) -> Result<Vec<BlockHash>, Box<dyn Error>> {
        let hash = self.rpc.generate_to_address(block_num, address)?;
        Ok(hash)
    }

    fn get_raw_mempool_verbose(
        &self,
    ) -> Result<HashMap<Txid, GetMempoolEntryResult>, Box<dyn Error>> {
        let raw_mempool = self.rpc.get_raw_mempool_verbose()?;
        Ok(raw_mempool)
    }

    fn send_to_address(
        &self,
        address: &Address<NetworkChecked>,
        amount: Amount,
        comment: Option<&str>,
        comment_to: Option<&str>,
        subtract_fee: Option<bool>,
        replaceable: Option<bool>,
        confirmation_target: Option<u32>,
        estimate_mode: Option<EstimateMode>,
    ) -> Result<Txid, Box<dyn Error>> {
        let tx_hash = self.rpc.send_to_address(
            address,
            amount,
            comment,
            comment_to,
            subtract_fee,
            replaceable,
            confirmation_target,
            estimate_mode,
        )?;
        Ok(tx_hash)
    }

    fn get_node_addresses(
        &self,
        count: Option<usize>,
    ) -> Result<Vec<GetNodeAddressesResult>, Box<dyn Error>> {
        let address = self.rpc.get_node_addresses(count)?;
        Ok(address)
    }

    fn estimate_smart_fee(
        &self,
        conf_target: u16,
        estimate_mode: Option<EstimateMode>,
    ) -> Result<EstimateSmartFeeResult, Box<dyn Error>> {
        let fee_result = self.rpc.estimate_smart_fee(conf_target, estimate_mode)?;
        Ok(fee_result)
    }

    fn wallet_create_funded_psbt(
        &self,
        inputs: &[CreateRawTransactionInput],
        outputs: &HashMap<String, Amount>,
        locktime: Option<i64>,
        options: Option<WalletCreateFundedPsbtOptions>,
        bip32derivs: Option<bool>,
    ) -> Result<WalletCreateFundedPsbtResult, Box<dyn Error>> {
        let psbt_result =
            self.rpc
                .wallet_create_funded_psbt(inputs, outputs, locktime, options, bip32derivs)?;
        Ok(psbt_result)
    }

    fn wallet_process_psbt(
        &self,
        psbt: &str,
        sign: Option<bool>,
        sighash_type: Option<SigHashType>,
        bip32derivs: Option<bool>,
    ) -> Result<WalletProcessPsbtResult, Box<dyn Error>> {
        let psbt_result = self
            .rpc
            .wallet_process_psbt(psbt, sign, sighash_type, bip32derivs)?;
        Ok(psbt_result)
    }

    fn rescan_blockchain(
        &self,
        start_from: Option<usize>,
        stop_height: Option<usize>,
    ) -> Result<(usize, Option<usize>), Box<dyn Error>> {
        let rescan = self.rpc.rescan_blockchain(start_from, stop_height)?;
        Ok(rescan)
    }

    fn get_tx_out_set_info(
        &self,
        hash_type: Option<TxOutSetHashType>,
        hash_or_height: Option<HashOrHeight>,
        use_index: Option<bool>,
    ) -> Result<GetTxOutSetInfoResult, Box<dyn Error>> {
        let result = self
            .rpc
            .get_tx_out_set_info(hash_type, hash_or_height, use_index)?;
        Ok(result)
    }

    fn scan_tx_out_set_blocking(
        &self,
        descriptors: &[ScanTxOutRequest],
    ) -> Result<ScanTxOutResult, Box<dyn Error>> {
        let tx_out = self.rpc.scan_tx_out_set_blocking(descriptors)?;
        Ok(tx_out)
    }
}

fn take_input(message: &str) -> String {
    println!("Please enter {message} : ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // let rpc_url = take_input("rpc url");
    // let rpc_user = take_input("rpc user");
    // let rpc_password = take_input("rpc password");

    // let client = Clients::new(
    //     &rpc_url,     // "http://localhost:8332",
    //     rpc_user,     // "abc".to_string(),
    //     rpc_password, // "abc".to_string(),
    // )?;
    let client = Clients::new(
        "http://localhost:8332",
        "abc".to_string(),
        "abc".to_string(),
    )?;

    loop {
        client.selector.print_all_keys();
        let key = take_input("Type ").to_lowercase();

        if !client.selector.get_keys().contains(&key.trim()) {
            println!("\ninvalid type {}", key);
            println!("\nSelect type from");
            continue;
        }

        client.selector.print_with_key(key.trim());
        let value = client.selector.get_values(key.trim());
        let user_selected_function = take_input("function you want to work with ").to_lowercase();
        let function_name = match value {
            Some(value) => {
                if !value
                    .iter()
                    .any(|&s| s.contains(user_selected_function.trim()))
                {
                    // if !value.contains(&user_selected_function.trim()) {
                    println!("\ninvalid function {}\n", user_selected_function);
                    continue;
                }
                user_selected_function.trim()
            }
            None => unreachable!(),
        };

        if function_name == "get_block_count" {
            let data = client.get_block_count();
            println!("Current Block Count : {}", data);
        } else if function_name == "get_latest_block_hash" {
            let data = client.get_latest_block_hash().unwrap();
            println!("Current Block Hash : {}", data);
        } else if function_name == "get_block" {
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            let block = client.get_block(block_hash).unwrap();
            prettify_data(block);
        } else if function_name == "get_best_block" {
            let block = client.get_best_block().unwrap();
            prettify_data(block);
        } else if function_name == "get_network_info" {
            let data = client.get_network_info().unwrap();
            prettify_data(data);
        } else if function_name == "get_index_info" {
            let data = client.get_index_info().unwrap();
            prettify_data(data);
        } else if function_name == "version" {
            let data = client.version().unwrap();
            prettify_data(data);
        // } else if function_name == "load_wallet" {
        //     let data = client.load_wallet().unwrap();
        // } else if function_name == "unload_wallet" {
        //     let data = client.unload_wallet().unwrap();
        } else if function_name == "list_wallets" {
            let data = client.list_wallets().unwrap();
            for wallet in data.iter() {
                println!("{}", wallet);
            }
        } else if function_name == "list_wallet_dir" {
            let data = client.list_wallet_dir().unwrap();
            for wallet_dir in data.iter() {
                println!("{}", wallet_dir);
            }
        } else if function_name == "get_wallet_info" {
            let data = client.get_wallet_info().unwrap();
            prettify_data(data);
        // } else if function_name == "backup_wallet" {
        //     let data = client.backup_wallet().unwrap();
        } else if function_name == "dump_private_key" {
            let address = get_address();
            let p_key = client.dump_private_key(address).unwrap();
            println!("{}", p_key);
        } else if function_name == "encrypt_wallet" {
            let address = take_input("passphrase");
            client.encrypt_wallet(&address).unwrap();
        } else if function_name == "get_difficulty" {
            let data = client.get_difficulty().unwrap();
            println!("{}", data);
        } else if function_name == "get_connection_count" {
            let data = client.get_connection_count().unwrap();
            println!("{}", data);
        } else if function_name == "get_block_hex" {
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            let data = client.get_block_hex(block_hash).unwrap();
            println!("{}", data);
        } else if function_name == "get_block_info" {
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            let block = client.get_block_info(block_hash).unwrap();
            prettify_data(block);
        } else if function_name == "get_block_header" {
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            let block = client.get_block_header(block_hash).unwrap();
            prettify_data(block);
        } else if function_name == "get_mining_info" {
            let data = client.get_mining_info().unwrap();
            prettify_data(data);
        } else if function_name == "get_blockchain_info" {
            let data = client.get_blockchain_info().unwrap();
            prettify_data(data);
        } else if function_name == "get_best_block_hash" {
            let data = client.get_best_block_hash().unwrap();
            println!("{}", data);
        } else if function_name == "get_block_hash" {
            let height = get_block_number();
            let data = client.get_block_hash(height).unwrap();
            println!("{}", data);
        } else if function_name == "get_block_stats" {
            let height = get_block_number();
            let data = client.get_block_stats(height).unwrap();
            prettify_data(data);
        } else if function_name == "get_balances" {
            let data = client.get_balances().unwrap();
            prettify_data(data);
        } else if function_name == "get_received_by_address" {
            let address = get_address();
            let min_conf = take_input("minimum confirmation").parse().unwrap();

            let data = client
                .get_received_by_address(&address, Some(min_conf))
                .unwrap();
            println!("{}", data);
        } else if function_name == "set_label" {
            let address = get_address();
            let label = take_input("label");

            client.set_label(&address, label.trim()).unwrap();
        } else if function_name == "key_pool_refill" {
            let size = take_input("new_size").parse().unwrap();
            client.key_pool_refill(Some(size)).unwrap();
        // } else if function_name == "lock_unspent" {
        //     let data = client.lock_unspent().unwrap();
        //     println!("{}", data);
        // } else if function_name == "unlock_unspent" {
        //     let data = client.unlock_unspent().unwrap();
        //     println!("{}", data);
        } else if function_name == "unlock_unspent_all" {
            let data = client.unlock_unspent_all().unwrap();
            println!("{}", data);
        } else if function_name == "stop" {
            client.stop().unwrap();
        // } else if function_name == "get_raw_change_address" {
        //     let data = client.get_raw_change_address().unwrap();
        //     println!("{}", data);
        } else if function_name == "get_address_info" {
            let address = get_address();
            let data = client.get_address_info(&address).unwrap();
            prettify_data(data);
        } else if function_name == "generate" {
            let block_number = take_input("block number").parse().unwrap();
            let max_tries = take_input("max tries").parse().unwrap();
            let data = client.generate(block_number, Some(max_tries)).unwrap();
            print_object(data);
        } else if function_name == "invalidate_block" {
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            client.invalidate_block(block_hash).unwrap();
        } else if function_name == "reconsider_block" {
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            client.reconsider_block(block_hash).unwrap();
        } else if function_name == "get_mempool_entry" {
            let tx_id = take_input("transaction id ").parse().unwrap();
            let data = client.get_mempool_entry(tx_id).unwrap();
            prettify_data(data);
        } else if function_name == "get_raw_mempool" {
            let data = client.get_raw_mempool().unwrap();
            print_object(data);
        } else if function_name == "get_chain_tips" {
            let data = client.get_chain_tips().unwrap();
            print_object(data);
        } else if function_name == "add_node" {
            let node_address = get_node_address();

            client.add_node(&node_address).unwrap();
        } else if function_name == "remove_node" {
            let node_address = get_node_address();
            client.remove_node(&node_address).unwrap();
        } else if function_name == "onetry_node" {
            let node_address = get_node_address();
            client.onetry_node(&node_address).unwrap();
        } else if function_name == "disconnect_node" {
            let node_address = get_node_address();
            client.disconnect_node(&node_address).unwrap();
        } else if function_name == "disconnect_node_by_id" {
            let node_id = take_input("node id ").parse().unwrap();

            client.disconnect_node_by_id(node_id).unwrap();
        } else if function_name == "get_added_node_info" {
            let node = take_input("node ");
            let data = client.get_added_node_info(Some(&node)).unwrap();
            print_object(data);
        } else if function_name == "list_banned" {
            let data = client.list_banned().unwrap();
            print_object(data);
        } else if function_name == "clear_banned" {
            client.clear_banned().unwrap();
        // } else if function_name == "add_ban" {
        //     // subnet: &str, bantime: u64, absolute: bool
        //     client.add_ban().unwrap();
        // } else if function_name == "remove_ban" {
        //     client.remove_ban().unwrap();
        } else if function_name == "set_network_active" {
            let input = get_bool("\n0. for false \n1. for true");
            let data = client.set_network_active(input).unwrap();
            println!("{}", data);
        } else if function_name == "get_peer_info" {
            let data = client.get_peer_info().unwrap();
            print_object(data);
        } else if function_name == "ping" {
            client.ping().unwrap();
        // } else if function_name == "send_raw_transaction" {
        //     let data = client.send_raw_transaction().unwrap();
        //     println!("{}", data);
        } else if function_name == "wait_for_new_block" {
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            let timeout = take_input("time out").parse().unwrap();
            let data = client.wait_for_new_block(block_hash, timeout).unwrap();
            prettify_data(data);
        } else if function_name == "get_descriptor_info" {
            let input = take_input("desc");
            let data = client.get_descriptor_info(&input).unwrap();
            prettify_data(data);
        } else if function_name == "join_psbt" {
            let data = get_string_array("psbt array");
            let data = client.join_psbt(&data).unwrap();
            println!("{}", data);
        } else if function_name == "combine_psbt" {
            let data = get_string_array("psbt array");
            let data = client.combine_psbt(&data).unwrap();
            println!("{}", data);
        } else if function_name == "combine_raw_transaction" {
            let data = get_string_array("raw transaction array");
            let data = client.combine_raw_transaction(&data).unwrap();
            println!("{}", data);
        } else if function_name == "finalize_psbt" {
            let psbt = take_input("psbt");
            let input = get_bool("do you want to extract?");
            let data = client.finalize_psbt(&psbt, Some(input)).unwrap();
            prettify_data(data);
        // } else if function_name == "derive_addresses" {
        //     let data = client.derive_addresses().unwrap();
        //     print_object(data);
        } else if function_name == "get_net_totals" {
            let data = client.get_net_totals().unwrap();
            prettify_data(data);
        } else if function_name == "get_network_hash_ps" {
            let nblocks = take_input("psbt").parse().unwrap();
            let height = take_input("psbt").parse().unwrap();
            let data = client
                .get_network_hash_ps(Some(nblocks), Some(height))
                .unwrap();
            println!("{}", data);
        } else if function_name == "uptime" {
            let data = client.uptime().unwrap();
            println!("{}", data);
        // } else if function_name == "submit_block" {
        //     client.submit_block().unwrap();
        // } else if function_name == "submit_block_bytes" {
        //     client.submit_block_bytes().unwrap();
        } else if function_name == "submit_block_hex" {
            let nblocks = take_input("block hex");

            client.submit_block_hex(nblocks.trim()).unwrap();
        // } else if function_name == "add_multisig_address" {
        //     let data = client.add_multisig_address().unwrap();
        //     prettify_data(data);
        } else if function_name == "create_wallet" {
            let wallet = take_input("wallet");
            let disable_private_keys = get_bool("disable private keys");
            let blank = get_bool("blank");
            let passphrase = take_input("passphrase");
            let avoid_reuse = get_bool("avoid reuse");

            let data = client
                .create_wallet(
                    wallet.trim(),
                    Some(disable_private_keys),
                    Some(blank),
                    Some(passphrase.trim()),
                    Some(avoid_reuse),
                )
                .unwrap();
            prettify_data(data);
        } else if function_name == "get_block_header_info" {
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            let data = client.get_block_header_info(&block_hash).unwrap();
            prettify_data(data);
        // } else if function_name == "get_block_template" {
        //     let data = client.get_block_template().unwrap();
        //     prettify_data(data);
        // } else if function_name == "get_block_stats_fields" {
        //     let data = client.get_block_stats_fields().unwrap();
        //     prettify_data(data);
        } else if function_name == "get_raw_transaction" {
            let tx_id = take_input("transaction id ").parse().unwrap();
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            let data = client
                .get_raw_transaction(&tx_id, Some(&block_hash))
                .unwrap();
            prettify_data(data);
        } else if function_name == "get_raw_transaction_hex" {
            let tx_id = take_input("transaction id ").parse().unwrap();
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            let data = client
                .get_raw_transaction_hex(&tx_id, Some(&block_hash))
                .unwrap();
            println!("{}", data);
        } else if function_name == "get_raw_transaction_info" {
            let tx_id = take_input("transaction id ").parse().unwrap();
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            let data = client
                .get_raw_transaction_info(&tx_id, Some(&block_hash))
                .unwrap();
            prettify_data(data);
        } else if function_name == "get_block_filter" {
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            let data = client.get_block_filter(&block_hash).unwrap();
            prettify_data(data);
        } else if function_name == "get_balance" {
            let min_conf = take_input("minimum confirmation").parse().unwrap();
            let include_watchonly = get_bool("include_watchonly");
            let data = client
                .get_balance(Some(min_conf), Some(include_watchonly))
                .unwrap();
            prettify_data(data);
        } else if function_name == "get_transaction" {
            let tx_id = take_input("transaction id ").parse().unwrap();
            let include_watchonly = get_bool("include_watchonly");
            let data = client
                .get_transaction(&tx_id, Some(include_watchonly))
                .unwrap();
            prettify_data(data);
        } else if function_name == "list_transactions" {
            let label = take_input("label");
            let count = take_input("count").parse().unwrap();
            let skip = take_input("skip").parse().unwrap();
            let include_watchonly = get_bool("include_watchonly");

            let data = client
                .list_transactions(
                    Some(label.trim()),
                    Some(count),
                    Some(skip),
                    Some(include_watchonly),
                )
                .unwrap();
            print_object(data);
        } else if function_name == "list_since_block" {
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            let target_confirmations = take_input("count").parse().unwrap();
            let include_watchonly = get_bool("include_watchonly");
            let include_removed = get_bool("include_removed");
            let data = client
                .list_since_block(
                    Some(&block_hash),
                    Some(target_confirmations),
                    Some(include_watchonly),
                    Some(include_removed),
                )
                .unwrap();
            prettify_data(data);
        } else if function_name == "get_tx_out_proof" {
            let tx_id = get_txid_array("transaction id array");
            let height = get_block_number();
            let block_hash = client.get_block_hash(height).unwrap();
            let data = client.get_tx_out_proof(&tx_id, Some(&block_hash)).unwrap();
            print_object(data);
        // } else if function_name == "import_public_key" {
        //     client.import_public_key().unwrap();
        // } else if function_name == "import_private_key" {
        //     client.import_private_key().unwrap();
        // } else if function_name == "import_address" {
        //     client.import_address().unwrap();
        // } else if function_name == "import_address_script" {
        //     client.import_address_script().unwrap();
        // } else if function_name == "import_multi" {
        //     let data = client.import_multi().unwrap();
        //     print_object(data);
        // } else if function_name == "import_descriptors" {
        //     let data = client.import_descriptors().unwrap();
        //     print_object(data);
        // } else if function_name == "list_unspent" {
        //     let data = client.list_unspent().unwrap();
        //     print_object(data);
        } else if function_name == "list_received_by_address" {
            let address_filter = get_address();
            let min_conf = take_input("minimum confirmation").parse().unwrap();
            let include_empty = get_bool("include empty");
            let include_watchonly = get_bool("include watchonly");

            let data = client
                .list_received_by_address(
                    Some(&address_filter),
                    Some(min_conf),
                    Some(include_empty),
                    Some(include_watchonly),
                )
                .unwrap();
            print_object(data);
        // } else if function_name == "create_psbt" {
        //     let data = client.create_psbt().unwrap();
        //     println!("{}", data);
        // } else if function_name == "create_raw_transaction_hex" {
        //     let data = client.create_raw_transaction_hex().unwrap();
        //     println!("{}", data);
        // } else if function_name == "create_raw_transaction" {
        //     let data = client.create_raw_transaction().unwrap();
        //     prettify_data(data);
        } else if function_name == "decode_raw_transaction" {
            let tx = take_input("enter transaction");
            let is_witness = get_bool("is witness");

            let data = client
                .decode_raw_transaction(tx.trim(), Some(is_witness))
                .unwrap();
            prettify_data(data);
        // } else if function_name == "fund_raw_transaction" {
        //     let data = client.fund_raw_transaction().unwrap();
        //     prettify_data(data);
        // } else if function_name == "sign_raw_transaction" {
        //     let data = client.sign_raw_transaction().unwrap();
        //     prettify_data(data);
        // } else if function_name == "sign_raw_transaction_with_wallet" {
        //     let data = client.sign_raw_transaction_with_wallet().unwrap();
        //     prettify_data(data);
        // } else if function_name == "sign_raw_transaction_with_key" {
        //     let data = client.sign_raw_transaction_with_key().unwrap();
        //     prettify_data(data);
        } else if function_name == "verify_message" {
            let address = get_address();
            let signature = take_input("signature").parse().unwrap();
            let message = take_input("message");

            let data = client
                .verify_message(&address, &signature, message.trim())
                .unwrap();
            println!("{}", data);
        } else if function_name == "get_new_address" {
            let label = take_input("label");
            let data = client.get_new_address(Some(label.trim()), None).unwrap();
            prettify_data(data);
        } else if function_name == "generate_to_address" {
            let height = get_block_number();
            let address = get_address();
            let data = client.generate_to_address(height, &address).unwrap();
            print_object(data);
        // } else if function_name == "get_raw_mempool_verbose" {
        //     let data = client.get_raw_mempool_verbose().unwrap();
        //     print_hashmap(data);
        // } else if function_name == "send_to_address" {
        //     let data = client.send_to_address().unwrap();
        //     prettify_data(data);
        } else if function_name == "get_node_addresses" {
            let count = take_input("count").trim().parse().unwrap();
            let data = client.get_node_addresses(Some(count)).unwrap();
            print_object(data);
        // } else if function_name == "estimate_smart_fee" {
        //     let data = client.estimate_smart_fee().unwrap();
        //     prettify_data(data);
        // } else if function_name == "wallet_create_funded_psbt" {
        //     let data = client.wallet_create_funded_psbt().unwrap();
        //     prettify_data(data);
        // } else if function_name == "wallet_process_psbt" {
        //     let data = client.wallet_process_psbt().unwrap();
        //     prettify_data(data);
        } else if function_name == "rescan_blockchain" {
            let start_from = take_input("start from").trim().parse().unwrap();
            let stop_height = take_input("stop from").trim().parse().unwrap();
            let data = client
                .rescan_blockchain(Some(start_from), Some(stop_height))
                .unwrap();
            println!("{:?}", data);
            // } else if function_name == "get_tx_out_set_info" {
            //     let data = client.get_tx_out_set_info().unwrap();
            //     prettify_data(data);
            // } else if function_name == "scan_tx_out_set_blocking" {
            //     let data = client.scan_tx_out_set_blocking().unwrap();
            //     prettify_data(data);
        }
    }
}
