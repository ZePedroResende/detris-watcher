pub mod auth;
pub mod block;

use crate::slack;
use structopt::StructOpt;
use tracing::info;

use ethers::{
    abi::AbiDecode,
    prelude::{StreamExt, U256},
};
use eyre::Result as EyreResult;

#[derive(Debug, PartialEq, Eq, StructOpt)]
pub struct Options {
    #[structopt(flatten)]
    provider: auth::Options,
    #[structopt(flatten)]
    slack:    slack::Options,
}

pub async fn main(options: Options) -> EyreResult<()> {
    // ethereum provider
    info!("Starting provider");
    let provider = auth::provider(options.provider).await?;
    let mut stream = block::mint_stream(&provider).await?;

    let (client, token) = slack::connect(&options.slack);
    let session = slack::session(&client, &token);

    info!("Starting to stream mint events !");

    // stream mints and send slack message
    while let Some(log) = stream.next().await {
        info!("block: id: {:?}", log.block_number);
        let id: String = U256::decode(log.topics[3])?.to_string().clone();
        let minter = log.topics[2].to_string();

        info!(channel=%options.slack.channel,"Id : {} minted by : {}", id, minter);

        slack::send_mint_notification(&session, &options.slack.channel, id, minter).await?;
    }

    Ok(())
}
