mod message;

use crate::ethereum::auth::{provider, Config};
use crate::ethereum::block::mint_stream;
use ethers::abi::AbiDecode;
use ethers::prelude::*;
use eyre::Result as EyreResult;
use tracing::info;

use message::MintMessageTemplateParams;

use slack_morphism::prelude::*;
use slack_morphism_hyper::*;

pub async fn send_mint_notification(config: Config, slack_token: String) -> EyreResult<()> {
    // ethereum provider
    let provider = provider(config).await?;
    let mut stream = mint_stream(&provider).await?;

    info!("Creating slack client");
    // slack client
    let client = SlackClient::new(SlackClientHyperConnector::new());

    info!("Creating slack api token");
    let token: SlackApiToken = SlackApiToken::new(slack_token.into());

    info!("Opening slack session");
    // Sessions are lightweight and basically just a reference to client and token
    let session = client.open_session(&token);

    info!("Starting to stream mint events !");

    //stream mints and send slack message
    while let Some(log) = stream.next().await {
        info!("block: id: {:?}", log.block_number);
        let id: String = U256::decode(log.topics[3])?.to_string().to_owned();
        let minter = log.topics[2].to_string();

        info!("Id : {} minted by : {}", id, minter);

        let message = MintMessageTemplateParams { id, minter };

        let post_chat_req =
            SlackApiChatPostMessageRequest::new("#tarot".into(), message.render_template());

        session.chat_post_message(&post_chat_req).await?;
    }

    Ok(())
}
