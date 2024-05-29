use ethers::{
    middleware::{
        gas_escalator::{Frequency, GasEscalatorMiddleware, GeometricGasPrice},
        gas_oracle::{GasNow, GasOracleMiddleware},
        MiddlewareBuilder, NonceManagerMiddleware, SignerMiddleware,
    },
    prelude::*,
    utils::AnvilInstance,
};
use gas_oracle::ProviderOracle;
use std::error::Error;

pub struct EtherClient {
    provider: Provider<Http>,
    client: Option<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl EtherClient {
    pub fn new(rpc: String) -> Result<EtherClient, Box<dyn Error>> {
        let provider: Provider<Http> = Provider::<Http>::try_from(rpc)?;

        Ok(Self {
            provider,
            client: None,
        })
    }

    pub fn set_client_with_privet_key(
        &mut self,
        wallet: LocalWallet,
    ) -> Result<(), Box<dyn Error>> {
        let value: SignerMiddleware<Provider<Http>, LocalWallet> =
            SignerMiddleware::new(self.provider.clone(), wallet);
        self.client = Some(value);

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

    pub async fn create_and_send_tx(&self, to: H160, value: U256) -> Result<(), Box<dyn Error>> {
        // let user_account = self.client.clone().unwrap().address();

        // let tx = TransactionRequest::new()
        //     .from(user_account)
        //     .to(to)
        //     .value(value);

        // let nonce_manager = self.client.clone().unwrap().nonce_manager(user_account);

        // let tx_recipet = nonce_manager
        //     .send_transaction(tx, Some(BlockNumber::Pending.into()))
        //     .await?
        //     .await?
        //     .unwrap_or(TransactionReceipt::default());

        // println!("tx_recipet {tx_recipet:?}");

        let client = self.client.clone().ok_or("Client not initialized")?;
        let user_account = client.address();

        let tx = TransactionRequest::new()
            .from(user_account)
            .to(to)
            .value(value);

        println!("{tx:?}");

        let nonce_manager = client.nonce_manager(user_account);

        let pending_tx = nonce_manager
            .send_transaction(tx, Some(BlockNumber::Pending.into()))
            .await?;

        match pending_tx.await {
            Ok(Some(tx_receipt)) => {
                println!("Transaction receipt: {:?}", tx_receipt);
            }
            Ok(None) => {
                println!("Transaction was not included in a block yet.");
            }
            Err(err) => {
                println!("Error waiting for transaction receipt: {:?}", err);
                return Err(Box::new(err));
            }
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

    pub async fn send_coin(
        &self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<Bytes, Box<dyn Error>> {
        let tx = TransactionRequest::default()
            .from(from)
            .to(to)
            .value(value)
            .into();
        let result = self.provider.call_raw(&tx).await?;

        Ok(result)
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
}
