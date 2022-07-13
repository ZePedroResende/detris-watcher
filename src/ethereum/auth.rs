// use ethers::core::types::{Address, Filter, ValueOrArray, H256, U256};
use ethers::providers::{Authorization, Provider, Ws};
use eyre::Result;
use std::sync::Arc;
use tracing::info;

pub struct Config {
    pub url: String,
    pub username: String,
    pub password: String,
}

pub async fn provider(
    Config {
        url,
        username,
        password,
    }: Config,
) -> Result<Arc<Provider<Ws>>> {
    info!("Connecting to provider...");

    let provider: Provider<Ws> =
        Provider::<Ws>::connect_with_auth(url, Authorization::basic(username, password)).await?;
    let provider = Arc::new(provider);

    info!("Connected to provider !");

    Ok(provider)
}
