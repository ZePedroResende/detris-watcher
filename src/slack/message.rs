use slack_morphism::prelude::*;

#[derive(Debug, Clone)]
pub struct MintMessageTemplateParams {
    pub id:     String,
    pub minter: String,
}

impl SlackMessageTemplate for MintMessageTemplateParams {
    fn render_template(&self) -> SlackMessageContent {
        SlackMessageContent::new()
            .with_text(format!("Hey id {} was minted by: {}", self.id, self.minter))
    }
}
