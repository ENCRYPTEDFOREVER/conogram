use serde::Serialize;

use crate::entities::{input_rich_block::InputRichBlock, rich_text::RichText};

/// A block quotation, corresponding to the HTML tag `<blockquote>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockblockquotation)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "blockquote", tag = "type")]
pub struct InputRichBlockBlockQuotation {
    /// Content of the block
    pub blocks: Vec<InputRichBlock>,

    /// *Optional*. Credit of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<Box<RichText>>,
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::misc::input_file::GetFiles;

impl GetFiles for InputRichBlockBlockQuotation {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        let mut form = form;
        for block in &self.blocks {
            form = block.form(form).await?;
        }
        Ok(form)
    }
}
