use serde::Serialize;

use crate::{
    entities::{input_rich_block::InputRichBlock, rich_block_list_item::RichBlockListItemType},
    utils::deserialize_utils::is_false,
};

/// An item of a list to be sent.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblocklistitem)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InputRichBlockListItem {
    /// The content of the item
    pub blocks: Vec<InputRichBlock>,

    /// *Optional*. Pass *True* if the item has a checkbox
    #[serde(skip_serializing_if = "is_false")]
    pub has_checkbox: bool,

    /// *Optional*. Pass *True* if the item has a checked checkbox
    #[serde(skip_serializing_if = "is_false")]
    pub is_checked: bool,

    /// *Optional*. For ordered lists, the numeric value of the item label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,

    /// *Optional*. For ordered lists, the type of the item label; must be one of “a” for lowercase letters, “A” for uppercase letters, “i” for lowercase Roman numerals, “I” for uppercase Roman numerals, or “1” for decimal numbers
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<RichBlockListItemType>,
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::misc::input_file::GetFiles;

impl GetFiles for InputRichBlockListItem {
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
