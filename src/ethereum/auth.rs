// use ethers::core::types::{Address, Filter, ValueOrArray, H256, U256};
use ethers::providers::{Authorization, Provider, Ws};
use eyre::Result;
use std::sync::Arc;
use structopt::StructOpt;
use tracing::info;

#[derive(Debug, PartialEq, Eq, StructOpt)]
pub struct Options {
    #[structopt(long, env = "PROVIDER_URL", default_value = "wss://bob.finiam.com")]
    pub provider_url:      String,
    #[structopt(long, env = "PROVIDER_USERNAME")]
    pub provider_username: String,
    #[structopt(long, env = "PROVIDER_PASSWORD")]
    pub provider_password: String,
}

pub async fn provider(
    Options {
        provider_url,
        provider_username,
        provider_password,
    }: Options,
) -> Result<Arc<Provider<Ws>>> {
    info!("Connecting to provider...");

    let provider: Provider<Ws> = Provider::<Ws>::connect_with_auth(
        provider_url,
        Authorization::basic(provider_username, provider_password),
    )
    .await?;
    let provider = Arc::new(provider);

    info!("Connected to provider !");

    Ok(provider)
}
