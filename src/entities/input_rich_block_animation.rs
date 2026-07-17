use serde::Serialize;

use crate::entities::{
    input_media_animation::InputMediaAnimation, rich_block_caption::RichBlockCaption,
};

/// A block with an animation, corresponding to the HTML tag `<video>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockanimation)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "animation", tag = "type")]
pub struct InputRichBlockAnimation {
    /// The animation. Caption is ignored.
    pub animation: InputMediaAnimation,

    /// *Optional*. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::misc::input_file::GetFiles;

impl GetFiles for InputRichBlockAnimation {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        self.animation.form(form).await
    }
}
