use serde::Serialize;

use crate::entities::{input_media_video::InputMediaVideo, rich_block_caption::RichBlockCaption};

/// A block with a video, corresponding to the HTML tag `<video>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockvideo)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "video", tag = "type")]
pub struct InputRichBlockVideo {
    /// The video. Caption is ignored.
    pub video: InputMediaVideo,

    /// *Optional*. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::misc::input_file::GetFiles;

impl GetFiles for InputRichBlockVideo {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        self.video.form(form).await
    }
}
