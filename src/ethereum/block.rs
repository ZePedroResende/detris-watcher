use ethers::{
    prelude::*,
    providers::{Provider, Ws},
    utils::keccak256,
};
use eyre::Result;
use std::sync::Arc;
use tracing::info;

const DETRIS_CONTRACT: &str = "0xbdc105c068715d57860702da9fa0c5ead11fba51";
const DETRIS_BLOCK: u64 = 14_896_795;
const DETRIS_MINT_EVENT: &str = "Transfer(address,address,uint256)";

pub async fn mint_stream(
    provider: &Arc<Provider<Ws>>,
) -> Result<SubscriptionStream<Ws, ethers::prelude::Log>> {
    let block = if true {
        provider.get_block_number().await?
    } else {
        U64::from(DETRIS_BLOCK)
    };

    info!("Starting on block {}", block);

    let last_block = provider
        .get_block(BlockNumber::Number(block))
        .await?
        .unwrap()
        .number
        .unwrap();

    let address: Address = DETRIS_CONTRACT.parse()?;

    let erc721_transfer_filter = Filter::new()
        .from_block(last_block)
        .address(address)
        .topic0(ValueOrArray::Value(H256::from(keccak256(
            DETRIS_MINT_EVENT,
        ))));

    let stream = provider.subscribe_logs(&erc721_transfer_filter).await?;

    info!("Subscribed to logs");

    Ok(stream)
}
