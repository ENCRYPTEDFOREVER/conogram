use serde::Serialize;

use crate::entities::{input_media_audio::InputMediaAudio, rich_block_caption::RichBlockCaption};

/// A block with a music file, corresponding to the HTML tag `<audio>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockaudio)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "audio", tag = "type")]
pub struct InputRichBlockAudio {
    /// The audio. Caption is ignored.
    pub audio: InputMediaAudio,

    /// *Optional*. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::misc::input_file::GetFiles;

impl GetFiles for InputRichBlockAudio {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        self.audio.form(form).await
    }
}
