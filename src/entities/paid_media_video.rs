use serde::{Deserialize, Serialize};

use crate::entities::video::Video;

/// The paid media is a video.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#paidmediavideo)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaidMediaVideo {
    /// The video
    pub video: Video,
}

// Divider: all content below this line will be preserved after code regen
