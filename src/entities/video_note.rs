use crate::entities::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

///This object represents a [video message](https://telegram.org/blog/video-messages-and-telescope) (available in Telegram apps as of [v.4.0](https://telegram.org/blog/video-messages-and-telescope)).
///API Reference: [link](https://core.telegram.org/bots/api/#videonote)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct VideoNote {
    ///Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,

    ///Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,

    ///Video width and height (diameter of the video message) as defined by the sender
    pub length: i64,

    ///Duration of the video in seconds as defined by the sender
    pub duration: i64,

    ///*Optional*. Video thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,

    ///*Optional*. File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
// Divider: all content below this line will be preserved after code regen
