use ethers::{
    middleware::{MiddlewareBuilder, SignerMiddleware},
    prelude::*,
    utils::{parse_units, AnvilInstance, ParseUnits},
};
use gas_oracle::ProviderOracle;
use std::error::Error;

pub struct EtherClient {
    provider: Provider<Http>,
    client: Option<SignerMiddleware<Provider<Http>, LocalWallet>>,
    address: Option<H160>,
}

impl EtherClient {
    pub fn new(rpc: String) -> Result<EtherClient, Box<dyn Error>> {
        let provider: Provider<Http> = Provider::<Http>::try_from(rpc)?;

        Ok(Self {
            provider,
            client: None,
            address: None,
        })
    }

    pub fn set_client_with_privet_key(
        &mut self,
        wallet: LocalWallet,
        chain_id: u64,
    ) -> Result<(), Box<dyn Error>> {
        let value: Option<SignerMiddleware<Provider<Http>, LocalWallet>> = Some(
            SignerMiddleware::new(self.provider.clone(), wallet.with_chain_id(chain_id)),
        );

        (self.client, self.address) = match value {
            Some(value) => (Some(value.clone()), Some(value.address())),
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Intialization failed",
                )))
            }
        };

        Ok(())
    }

    pub fn load_wallet(
        &self,
        instance: Option<&AnvilInstance>,
        p_key: String,
    ) -> Result<LocalWallet, Box<dyn Error>> {
        if instance.is_none() && p_key.is_empty() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Privet key not found for provided instance",
            )));
        }
        let wallet: LocalWallet = match instance {
            Some(instance) => instance.keys()[0].clone().into(),
            None => p_key.parse::<LocalWallet>()?,
        };

        Ok(wallet)
    }

    pub fn create_raw_coin_tx(
        &self,
        to: &str,
        value: u64,
        to_unit: &str,
    ) -> Result<TransactionRequest, Box<dyn Error>> {
        let user_account = self.address.unwrap();

        let value: ParseUnits = match to_unit {
            "wei" => parse_units(value, "wei").unwrap(),
            "kwei" => parse_units(value, "kwei").unwrap(),
            "mwei" => parse_units(value, "mwei").unwrap(),
            "gwei" => parse_units(value, "gwei").unwrap(),
            "szabo" => parse_units(value, "szabo").unwrap(),
            "finney" => parse_units(value, "finney").unwrap(),
            "ether" => parse_units(value, "ether").unwrap(),
            _ => value.into(),
        };

        let value = U256::from(value);

        let to: H160 = to.parse().unwrap();
        let tx = TransactionRequest::new()
            .from(user_account)
            .to(to)
            .value(value);

        Ok(tx)
    }

    pub fn get_client(&self) -> Option<SignerMiddleware<Provider<Http>, LocalWallet>> {
        self.client.clone()
    }

    pub async fn send_raw_tx(&self, tx: TransactionRequest) -> Result<(), Box<dyn Error>> {
        let user_account = self.address.unwrap();
        let nonce_manager = self.client.clone().unwrap().nonce_manager(user_account);

        let pending_tx = nonce_manager.send_transaction(tx, None).await?.await?;

        if pending_tx.is_some() {
            println!(
                "Pending tx: {}",
                serde_json::to_string_pretty(&pending_tx.unwrap())?
            );
        }

        Ok(())
    }

    pub async fn get_block_count(&self) -> Result<U64, Box<dyn Error>> {
        Ok(self.provider.get_block_number().await?)
    }

    pub async fn get_chain_id(&self) -> Result<U256, Box<dyn Error>> {
        Ok(self.provider.get_chainid().await?)
    }

    pub async fn get_tx_pool(&self) -> Result<TxpoolContent, Box<dyn Error>> {
        Ok(self.provider.txpool_content().await?)
    }

    pub async fn get_block(&self, block_number: u64) -> Result<Block<H256>, Box<dyn Error>> {
        let block = self.provider.get_block(block_number).await?;

        let result = match block {
            Some(value) => Ok(value),
            None => Err(()),
        };

        Ok(result.unwrap())
    }

    pub async fn get_account_balance(&self, from: Address) -> Result<U256, Box<dyn Error>> {
        Ok(self.provider.get_balance(from, None).await?)
    }

    pub async fn get_gas_price_oracle(&self) -> Result<U256, Box<dyn Error>> {
        let oracle = ProviderOracle::new(self.provider.clone());

        let price: U256 = oracle.fetch().await?;
        Ok(price)
    }

    pub async fn get_code(&self, at: Address) -> Result<Bytes, Box<dyn Error>> {
        Ok(self.provider.get_code(at, None).await?)
    }

    pub async fn is_contract_exists(&self, at: Address) -> Result<bool, Box<dyn Error>> {
        let code = self.get_code(at).await?;

        Ok(code.len() > 0)
    }

    pub async fn get_slot_data(&self, at: Address, slot: TxHash) -> Result<H256, Box<dyn Error>> {
        let slot_data = self.provider.get_storage_at(at, slot, None).await?;

        Ok(slot_data)
    }

    pub async fn get_transaction_data(
        &self,
        transaction_hash: TxHash,
    ) -> Result<Option<Transaction>, Box<dyn Error>> {
        let transaction = self.provider.get_transaction(transaction_hash).await?;

        Ok(transaction)
    }

    pub async fn sign_message(
        &self,
        message: String,
        wallet: LocalWallet,
    ) -> Result<Signature, Box<dyn Error>> {
        let signature = wallet.sign_message(message.as_bytes()).await?;
        Ok(signature)
    }
}
