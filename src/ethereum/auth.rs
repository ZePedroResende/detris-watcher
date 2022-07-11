use ethers::contract::{Contract, EthEvent};
// use ethers::core::types::{Address, Filter, ValueOrArray, H256, U256};
use anyhow::{Context, Result};
use ethers::{
    abi::AbiDecode,
    prelude::*,
    providers::{Authorization, Provider, Ws},
    utils::keccak256,
};
use std::sync::Arc;

pub async fn provider(
    url: String,
    username: String,
    password: String,
) -> Result<Arc<Provider<Ws>>> {
    let provider: Provider<Ws> =
        Provider::<Ws>::connect_with_auth(url, Authorization::basic(username, password)).await?;
    let provider = Arc::new(provider);

    Ok(provider)
}
