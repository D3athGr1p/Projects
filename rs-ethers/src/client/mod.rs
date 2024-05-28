use crate::client::k256::Secp256k1;
use ethers::{
    middleware::{
        gas_escalator::{Frequency, GasEscalatorMiddleware, GeometricGasPrice},
        gas_oracle::{GasNow, GasOracleMiddleware},
        MiddlewareBuilder, NonceManagerMiddleware, SignerMiddleware,
    },
    prelude::*,
};
use gas_oracle::ProviderOracle;
use std::{env, error::Error};

const RPC: &str = "https://eth.llamarpc.com";

pub struct EtherClient {
    provider: Provider<Http>,
    client: Option<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl EtherClient {
    pub fn new() -> Result<EtherClient, Box<dyn Error>> {
        let provider: Provider<Http> = Provider::<Http>::try_from(RPC)?;

        Ok(Self {
            provider,
            client: None,
        })
    }

    pub async fn set_client(&mut self, p_key: String) -> Result<(), Box<dyn Error>> {
        let wallet: LocalWallet = p_key.parse::<LocalWallet>()?;
        let address = wallet.address();
        let gas_oracle = GasNow::new();

        // let check: NonceManagerMiddleware<
        //     SignerMiddleware<GasOracleMiddleware<Provider<Http>, GasNow>, LocalWallet
        // > = Provider::<Http>::try_from(RPC)
        //     .unwrap()
        //     .gas_oracle(gas_oracle)
        //     .with_signer(wallet.clone())
        //     .nonce_manager(address);

        let value = SignerMiddleware::new(self.provider.clone(), wallet);
        self.client = Some(value);

        Ok(())
    }

    pub fn create_tx(&self, to: H160, value: U256) -> Result<(), Box<dyn Error>> {
        let tx = TransactionRequest::new().to(to).value(value);
        println!("TX RAW IS {tx:?}");
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
}
