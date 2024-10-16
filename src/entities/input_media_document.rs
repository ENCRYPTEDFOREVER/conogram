use serde::Serialize;

use crate::{
    entities::{
        message_entity::MessageEntity,
        misc::input_file::{GetFiles, InputFile},
    },
    utils::deserialize_utils::is_false,
};

/// Represents a general file to be sent.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputmediadocument)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InputMediaDocument {
    /// File to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub media: InputFile,

    /// *Optional*. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,

    /// *Optional*. Caption of the document to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// *Optional*. Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// *Optional*. Disables automatic server-side content type detection for files uploaded using multipart/form-data. Always *True*, if the document is sent as part of an album.
    #[serde(skip_serializing_if = "is_false")]
    pub disable_content_type_detection: bool,
}

impl GetFiles for InputMediaDocument {
    fn get_files(&self) -> Vec<&InputFile> {
        let mut vec = Vec::with_capacity(2);
        vec.push(&self.media);
        if let Some(thumbnail) = &self.thumbnail {
            vec.push(thumbnail);
        }
        vec
    }
}
// Divider: all content below this line will be preserved after code regen
impl<T: Into<InputFile>> From<T> for InputMediaDocument {
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            ..Default::default()
        }
    }
}
