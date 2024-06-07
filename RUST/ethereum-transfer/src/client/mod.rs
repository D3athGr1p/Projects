use dotenv::dotenv;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use std::error::Error;
use std::thread;
use std::time::Duration;

const GAS_PRICE: u64 = 21000 * 5;
const MINIMUM_AMOUNT: f64 = 0.001;

struct EtherumTx {
    signer: NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>,
    from: Address,
    to: Address,
    minimum_amount: U256,
}

fn ether_to_wei(ether: f64) -> U256 {
    let wei_per_ether = U256::exp10(18); // 10^18
    let wei_value = ether * wei_per_ether.as_u64() as f64;
    U256::from(wei_value as u64)
}

impl EtherumTx {
    async fn new(p_key: String, to: String, rpc: String) -> Result<EtherumTx, Box<dyn Error>> {
        let wallet: LocalWallet = p_key.parse::<LocalWallet>()?;
        let signer_address: Address = wallet.address();

        let provider = Provider::<Http>::try_from(rpc)?;
        let chain_id = provider.get_chainid().await?;
        let signer = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id.as_u64()));
        let to: Address = to.parse()?;

        let minimum = ether_to_wei(MINIMUM_AMOUNT);

        let nonce_mgr = signer.nonce_manager(signer_address);

        Ok(EtherumTx {
            signer: nonce_mgr,
            from: signer_address,
            to,
            minimum_amount: minimum,
        })
    }

    async fn get_latest_balance(&self) -> Result<U256, Box<dyn Error>> {
        let balance = self.signer.get_balance(self.from, None).await?;
        Ok(balance)
    }

    async fn get_latest_gas_price(&self) -> Result<U256, Box<dyn Error>> {
        let gas_price = self.signer.get_gas_price().await?;
        Ok(gas_price)
    }

    async fn prepare_tx(
        &self,
        current_balance: U256,
        current_gas_price: U256,
    ) -> Result<Option<TransactionReceipt>, Box<dyn Error>> {
        let balance = if self.minimum_amount < current_balance {
            Some(current_balance)
        } else {
            None
        };

        if balance.is_none() {
            return Ok(None);
        }

        let gas_amount = U256::from(GAS_PRICE) * current_gas_price;

        println!("{gas_amount}");

        let balance = balance.unwrap() - gas_amount;

        let tx = TransactionRequest::new()
            .from(self.from)
            .to(self.to)
            .value(balance)
            .gas_price(current_gas_price);

        let pending_tx = self.signer.send_transaction(tx, None).await?.await?;

        Ok(pending_tx)
    }

    async fn start(&self) -> Result<(), Box<dyn Error>> {
        let mut counter: u64 = 0;
        loop {
            counter = counter + 1;
            println!("Running this for {counter} times");
            let current_balance = self.get_latest_balance().await?;
            let current_gas_price = self.get_latest_gas_price().await?;

            let pending_tx = self.prepare_tx(current_balance, current_gas_price).await?;

            if pending_tx.is_none() {
                thread::sleep(Duration::from_secs(1));
                continue;
            }

            let tx_receipt = pending_tx.unwrap();
            println!("{:?}", tx_receipt);

            thread::sleep(Duration::from_secs(1));
        }
    }
}

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let p_key = std::env::var("PRIVET_KEY")?;
    let rpc = std::env::var("RPC_URL")?;
    let to = std::env::var("TO")?;

    let ether_tx = EtherumTx::new(p_key, to, rpc).await?;

    ether_tx.start().await?;

    Ok(())
}
