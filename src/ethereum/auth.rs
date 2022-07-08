use ethers::contract::{Contract, EthEvent};
//use ethers::core::types::{Address, Filter, ValueOrArray, H256, U256};
use anyhow::{Context, Result};
use ethers::abi::AbiDecode;
use ethers::prelude::*;
use ethers::providers::{Provider, Ws};
use ethers::utils::keccak256;
use ethers::providers::Authorization::Basic;
use std::sync::Arc;

async fn provider() -> Result<Provider<Ws>> {
    let username = std::env::var("PROVIDER_USERNAME")?;
    let password = std::env::var("PROVIDER_PASSWORD")?;


    
    let provider: Provider<Ws> = 
        Provider::<Ws>::connect_with_auth("https://bob.finiam.com", Basic::new_client(username, password)).await?;
    let provider = Arc::new(provider);

    Ok(provider)

    //     .get_block()
    //     .await?
    //     .unwrap()
    //     .number
    //     .unwrap();

    // let address: Address = "0x5738379364fab26c7e044c02ded4ceef93333d84".parse()?;
    // let erc721_transfer_filter = Filter::new()
    //     .from_block(last_block)
    //     .address(address)
    //     .topic0(ValueOrArray::Value(H256::from(keccak256(
    //         "Transfer(address,address,uint256)",
    //     ))));

    // let mut stream = provider.subscribe_logs(&erc721_transfer_filter).await?;

    // while let Some(log) = stream.next().await {
    //     println!(
    //         "block: {:?}, tx: {:?}, token: {:?}, from: {:?}, to: {:?}, id: {:?}",
    //         log.block_number,
    //         log.transaction_hash,
    //         log.address,
    //         Address::from(log.topics[1]),
    //         Address::from(log.topics[2]),
    //         Address::from(log.topics[3]),
    //     );
    // }

    // Ok(())
}
