use serde::{Deserialize, Serialize};

use crate::entities::live_photo::LivePhoto;

/// The paid media is a [live photo](https://core.telegram.org/bots/api/#livephoto).
///
/// API Reference: [link](https://core.telegram.org/bots/api/#paidmedialivephoto)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "live_photo", tag = "type")]
pub struct PaidMediaLivePhoto {
    /// The photo
    pub live_photo: LivePhoto,
}

// Divider: all content below this line will be preserved after code regen
