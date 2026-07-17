use serde::Serialize;

use crate::entities::input_rich_block_list_item::InputRichBlockListItem;

/// A list of blocks, corresponding to the HTML tag `<ul>` or `<ol>` with multiple nested tags `<li>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblocklist)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "list", tag = "type")]
pub struct InputRichBlockList {
    /// Items of the list
    pub items: Vec<InputRichBlockListItem>,
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::misc::input_file::GetFiles;

impl GetFiles for InputRichBlockList {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        let mut form = form;
        for item in &self.items {
            form = item.form(form).await?;
        }
        Ok(form)
    }
}
