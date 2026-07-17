use serde::Serialize;

use crate::{
    entities::{input_rich_block::InputRichBlock, rich_text::RichText},
    utils::deserialize_utils::is_false,
};

/// An expandable block for details disclosure, corresponding to the HTML tag `<details>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockdetails)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "details", tag = "type")]
pub struct InputRichBlockDetails {
    /// Always shown summary of the block
    pub summary: Box<RichText>,

    /// Content of the block
    pub blocks: Vec<InputRichBlock>,

    /// *Optional*. Pass *True* if the content of the block is visible by default
    #[serde(skip_serializing_if = "is_false")]
    pub is_open: bool,
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::misc::input_file::GetFiles;

impl GetFiles for InputRichBlockDetails {
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
