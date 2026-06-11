use serde::{Deserialize, Serialize};

use crate::{
    entities::{rich_block_caption::RichBlockCaption, video::Video},
    utils::deserialize_utils::is_false,
};

/// A block with a video, corresponding to the HTML tag `<video>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockvideo)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichBlockVideo {
    /// The video
    pub video: Video,

    /// *Optional*. *True*, if the media preview is covered by a spoiler animation
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_spoiler: bool,

    /// *Optional*. Caption of the block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen
