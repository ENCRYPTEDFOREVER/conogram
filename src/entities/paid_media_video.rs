use crate::entities::video::Video;
use serde::{Deserialize, Serialize};

///The paid media is a video.
///API Reference: [link](https://core.telegram.org/bots/api/#paidmediavideo)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PaidMediaVideo {
    ///The video
    pub video: Video,
}
// Divider: all content below this line will be preserved after code regen
