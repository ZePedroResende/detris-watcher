mod message;

use crate::{
    ethereum,
    ethereum::{auth::provider, block::mint_stream},
};
use ethers::{abi::AbiDecode, prelude::*};
use eyre::Result as EyreResult;
use tracing::info;

use message::MintMessageTemplateParams;
use slack_morphism::prelude::*;
use slack_morphism_hyper::SlackClientHyperConnector;
use structopt::StructOpt;

#[derive(Debug, PartialEq, Eq, StructOpt)]
pub struct Options {
    #[structopt(long, env = "SLACK_CHANNEL")]
    pub slack_channel: String,
    #[structopt(long, env = "SLACK_TOKEN")]
    pub slack_token:   String,
    #[structopt(flatten)]
    provider:          ethereum::auth::Options,
}

pub async fn main(options: Options) -> EyreResult<()> {
    // ethereum provider
    info!("Starting provider");
    let provider = provider(options.provider).await?;
    let mut stream = mint_stream(&provider).await?;

    info!("Creating slack client");
    // slack client
    let client = SlackClient::new(SlackClientHyperConnector::new());

    info!("Creating slack api token");
    let token: SlackApiToken = SlackApiToken::new(options.slack_token.into());

    info!("Opening slack session");
    // Sessions are lightweight and basically just a reference to client and token
    let session = client.open_session(&token);

    info!("Starting to stream mint events !");

    // stream mints and send slack message
    while let Some(log) = stream.next().await {
        info!("block: id: {:?}", log.block_number);
        let id: String = U256::decode(log.topics[3])?.to_string().clone();
        let minter = log.topics[2].to_string();

        info!(channel=%options.slack_channel,"Id : {} minted by : {}", id, minter);

        let message = MintMessageTemplateParams { id, minter };

        let post_chat_req = SlackApiChatPostMessageRequest::new(
            options.slack_channel.as_str().into(),
            message.render_template(),
        );

        session.chat_post_message(&post_chat_req).await?;
    }

    Ok(())
}
