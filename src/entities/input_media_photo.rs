use serde::Serialize;

use crate::{
    entities::{
        message_entity::MessageEntity,
        misc::input_file::{GetFiles, InputFile},
    },
    utils::deserialize_utils::is_false,
};

/// Represents a photo to be sent.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputmediaphoto)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InputMediaPhoto {
    /// File to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub media: InputFile,

    /// *Optional*. Caption of the photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// *Optional*. Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// *Optional*. Pass *True*, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "is_false")]
    pub show_caption_above_media: bool,

    /// *Optional*. Pass *True* if the photo needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "is_false")]
    pub has_spoiler: bool,
}

impl GetFiles for InputMediaPhoto {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        self.media.form(form).await
    }
}
// Divider: all content below this line will be preserved after code regen
