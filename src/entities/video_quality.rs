use serde::{Deserialize, Serialize};

/// This object represents a video file of a specific quality.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#videoquality)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoQuality {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,

    /// Video width
    pub width: i64,

    /// Video height
    pub height: i64,

    /// Codec that was used to encode the video, for example, “h264”, “h265”, or “av01”
    pub codec: VideoQualityCodec,

    /// *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

/// Codec that was used to encode the video, for example, “h264”, “h265”, or “av01”
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoQualityCodec {
    /// `h265`
    #[default]
    #[serde(rename = "h265")]
    H265,

    /// `av01`
    #[serde(rename = "av01")]
    Av01,
}

// Divider: all content below this line will be preserved after code regen
