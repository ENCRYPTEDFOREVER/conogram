use serde::Serialize;

use crate::entities::{input_rich_block::InputRichBlock, rich_block_caption::RichBlockCaption};

/// A slideshow, corresponding to the custom HTML tag `<tg-slideshow>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockslideshow)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "slideshow", tag = "type")]
pub struct InputRichBlockSlideshow {
    /// Elements of the slideshow
    pub blocks: Vec<InputRichBlock>,

    /// *Optional*. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::misc::input_file::GetFiles;

impl GetFiles for InputRichBlockSlideshow {
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
