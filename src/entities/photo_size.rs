use serde::{Deserialize, Serialize};

///This object represents one size of a photo or a [file](https://core.telegram.org/bots/api/#document) / [sticker](https://core.telegram.org/bots/api/#sticker) thumbnail.
///API Reference: [link](https://core.telegram.org/bots/api/#photosize)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PhotoSize {
    ///Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,

    ///Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,

    ///Photo width
    pub width: i64,

    ///Photo height
    pub height: i64,

    ///*Optional*. File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
// Divider: all content below this line will be preserved after code regen
