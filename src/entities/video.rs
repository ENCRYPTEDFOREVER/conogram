use serde::{Deserialize, Serialize};

use crate::entities::photo_size::PhotoSize;

/// This object represents a video file.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#video)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,

    /// Video width as defined by the sender
    pub width: i64,

    /// Video height as defined by the sender
    pub height: i64,

    /// Duration of the video in seconds as defined by the sender
    pub duration: i64,

    /// *Optional*. Video thumbnail
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,

    /// *Optional*. Original filename as defined by the sender
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    /// *Optional*. MIME type of the file as defined by the sender
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
