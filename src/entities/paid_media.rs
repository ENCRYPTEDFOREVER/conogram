use serde::{Deserialize, Serialize};

use crate::entities::{
    paid_media_live_photo::PaidMediaLivePhoto, paid_media_photo::PaidMediaPhoto,
    paid_media_preview::PaidMediaPreview, paid_media_video::PaidMediaVideo,
};

/// This object describes paid media. Currently, it can be one of
///
/// * [PaidMediaLivePhoto](https://core.telegram.org/bots/api/#paidmedialivephoto)
/// * [PaidMediaPhoto](https://core.telegram.org/bots/api/#paidmediaphoto)
/// * [PaidMediaPreview](https://core.telegram.org/bots/api/#paidmediapreview)
/// * [PaidMediaVideo](https://core.telegram.org/bots/api/#paidmediavideo)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#paidmedia)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PaidMedia {
    /// The paid media is a [live photo](https://core.telegram.org/bots/api/#livephoto).
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#paidmedialivephoto)
    #[serde(rename = "live_photo")]
    LivePhoto(PaidMediaLivePhoto),

    /// The paid media is a photo.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#paidmediaphoto)
    #[serde(rename = "photo")]
    Photo(PaidMediaPhoto),

    /// The paid media isn't available before the payment.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#paidmediapreview)
    #[serde(rename = "preview")]
    Preview(PaidMediaPreview),

    /// The paid media is a video.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#paidmediavideo)
    #[serde(rename = "video")]
    Video(PaidMediaVideo),
}

impl Default for PaidMedia {
    fn default() -> Self {
        Self::LivePhoto(PaidMediaLivePhoto::default())
    }
}

impl From<PaidMediaLivePhoto> for PaidMedia {
    fn from(value: PaidMediaLivePhoto) -> Self {
        Self::LivePhoto(value)
    }
}

impl From<PaidMediaPhoto> for PaidMedia {
    fn from(value: PaidMediaPhoto) -> Self {
        Self::Photo(value)
    }
}

impl From<PaidMediaPreview> for PaidMedia {
    fn from(value: PaidMediaPreview) -> Self {
        Self::Preview(value)
    }
}

impl From<PaidMediaVideo> for PaidMedia {
    fn from(value: PaidMediaVideo) -> Self {
        Self::Video(value)
    }
}

// Divider: all content below this line will be preserved after code regen
