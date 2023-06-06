use crate::entities::message_entity::MessageEntity;
use crate::entities::misc::input_file::InputFile;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;

///Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
///API Reference: [link](https://core.telegram.org/bots/api/#inputmediaanimation)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InputMediaAnimation {
    ///File to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub media: InputFile,

    ///*Optional*. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,

    ///*Optional*. Caption of the animation to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    ///*Optional*. Mode for parsing entities in the animation caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    ///*Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    ///*Optional*. Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    ///*Optional*. Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    ///*Optional*. Animation duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    ///*Optional*. Pass *True* if the animation needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_spoiler: bool,
}
// Divider: all content below this line will be preserved after code regen
use super::misc::input_file::GetFiles;
use super::misc::input_file::Moose;
use std::collections::HashMap;

impl GetFiles for InputMediaAnimation {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        map.insert(
            Moose::Owned(self.media.get_uuid().unwrap_or("media".into())),
            &self.media,
        );
        if let Some(thumb) = &self.thumbnail {
            map.insert(
                Moose::Owned(thumb.get_uuid().unwrap_or("thumbnail".into())),
                thumb,
            );
        }
        map
    }
}
