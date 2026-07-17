use serde::Serialize;

use crate::entities::{input_media_photo::InputMediaPhoto, rich_block_caption::RichBlockCaption};

/// A block with a photo, corresponding to the HTML tag `<img>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockphoto)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "photo", tag = "type")]
pub struct InputRichBlockPhoto {
    /// The photo. Caption is ignored.
    pub photo: InputMediaPhoto,

    /// *Optional*. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::misc::input_file::GetFiles;

impl GetFiles for InputRichBlockPhoto {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        self.photo.form(form).await
    }
}
