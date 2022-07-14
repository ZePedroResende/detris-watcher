mod message;

use eyre::Result as EyreResult;
use tracing::info;

use message::MintMessageTemplateParams;
use slack_morphism::{
    prelude::{SlackApiChatPostMessageRequest, SlackClientSession, SlackMessageTemplate},
    SlackApiToken, SlackApiTokenValue, SlackClient,
};
use slack_morphism_hyper::{SlackClientHyperConnector, SlackClientHyperHttpsConnector};
use structopt::StructOpt;

type Session<'a> = SlackClientSession<'a, SlackClientHyperHttpsConnector>;
type Client = SlackClient<SlackClientHyperHttpsConnector>;

#[derive(Debug, PartialEq, Eq, StructOpt)]
pub struct Options {
    #[structopt(long, env = "SLACK_CHANNEL")]
    pub channel: String,
    #[structopt(long, env = "SLACK_TOKEN")]
    pub token:   String,
}

pub fn connect(options: &Options) -> (Client, SlackApiToken) {
    info!("Creating slack client");
    // slack client
    let client = SlackClient::new(SlackClientHyperConnector::new());

    info!("Creating slack api token");
    let token_value = SlackApiTokenValue(options.token.clone());
    let token: SlackApiToken = SlackApiToken::new(token_value);

    (client, token)
}

pub fn session<'a>(client: &'a Client, token: &'a SlackApiToken) -> Session<'a> {
    info!("Opening slack session");
    // Sessions are lightweight and basically just a reference to client and token

    client.open_session(token)
}

pub async fn send_mint_notification<'a>(
    session: &'a Session<'a>,
    channel: &str,
    id: String,
    minter: String,
) -> EyreResult<()> {
    let message = MintMessageTemplateParams { id, minter };

    let post_chat_req =
        SlackApiChatPostMessageRequest::new(channel.into(), message.render_template());

    session.chat_post_message(&post_chat_req).await?;
    Ok(())
}
