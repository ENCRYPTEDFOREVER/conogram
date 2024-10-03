use serde::{Deserialize, Serialize};

use crate::entities::{
    paid_media_photo::PaidMediaPhoto, paid_media_preview::PaidMediaPreview,
    paid_media_video::PaidMediaVideo,
};

/// This object describes paid media. Currently, it can be one of
///
/// * [PaidMediaPreview](https://core.telegram.org/bots/api/#paidmediapreview)
/// * [PaidMediaPhoto](https://core.telegram.org/bots/api/#paidmediaphoto)
/// * [PaidMediaVideo](https://core.telegram.org/bots/api/#paidmediavideo)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#paidmedia)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PaidMedia {
    #[serde(rename = "preview")]
    Preview(PaidMediaPreview),

    #[serde(rename = "photo")]
    Photo(PaidMediaPhoto),

    #[serde(rename = "video")]
    Video(PaidMediaVideo),
}

impl Default for PaidMedia {
    fn default() -> Self {
        Self::Preview(PaidMediaPreview::default())
    }
}

impl From<PaidMediaPreview> for PaidMedia {
    fn from(value: PaidMediaPreview) -> Self {
        Self::Preview(value)
    }
}

impl From<PaidMediaPhoto> for PaidMedia {
    fn from(value: PaidMediaPhoto) -> Self {
        Self::Photo(value)
    }
}

impl From<PaidMediaVideo> for PaidMedia {
    fn from(value: PaidMediaVideo) -> Self {
        Self::Video(value)
    }
}

// Divider: all content below this line will be preserved after code regen
