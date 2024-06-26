use crate::helper::print_op;

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

pub struct Clients {
    rpc: Client,
}

impl Clients {
    fn new(
        rpc_url: &str,
        rpc_user: String,
        rpc_password: String,
    ) -> Result<Clients, Box<dyn Error>> {
        let rpc = Client::new(rpc_url, Auth::UserPass(rpc_user, rpc_password))?;

        Ok(Clients { rpc })
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
    let rpc_url = take_input("rpc url");
    let rpc_user = take_input("rpc user");
    let rpc_password = take_input("rpc password");

    let client = Clients::new(
        &rpc_url,     // "http://localhost:8332",
        rpc_user,     // "abc".to_string(),
        rpc_password, // "abc".to_string(),
    )?;

    loop {
        print_op::print_help_message();
        let op: u8 = take_input("enter your choice").parse().unwrap();
        match op {
            op if op < 9 => print_op::print_help(op),
            9 => {
                println!("Existing the system");
                break;
            }
            _ => {
                println!("Invalid choice: {}", op);
                continue;
            }
        }
    }

    Ok(())
}
