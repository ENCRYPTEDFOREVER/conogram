use serde::Serialize;

use crate::{
    entities::{
        message_entity::MessageEntity,
        misc::input_file::{GetFiles, InputFile},
    },
    utils::deserialize_utils::is_false,
};

/// Represents a live photo to be sent.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputmedialivephoto)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename = "live_photo", tag = "type")]
pub struct InputMediaLivePhoto {
    /// Video of the live photo to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended) or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Sending live photos by a URL is currently unsupported.
    pub media: InputFile,

    /// The static photo to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended) or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Sending live photos by a URL is currently unsupported.
    pub photo: InputFile,

    /// *Optional*. Caption of the live photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// *Optional*. Mode for parsing entities in the live photo caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// *Optional*. Pass *True* if the caption must be shown above the message media
    #[serde(skip_serializing_if = "is_false")]
    pub show_caption_above_media: bool,

    /// *Optional*. Pass *True* if the live photo needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "is_false")]
    pub has_spoiler: bool,
}

impl GetFiles for InputMediaLivePhoto {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        let mut form = form;
        form = self.media.form(form).await?;
        form = self.photo.form(form).await?;
        Ok(form)
    }
}
// Divider: all content below this line will be preserved after code regen
