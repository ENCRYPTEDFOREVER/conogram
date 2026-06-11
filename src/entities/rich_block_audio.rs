use serde::{Deserialize, Serialize};

use crate::entities::{audio::Audio, rich_block_caption::RichBlockCaption};

/// A block with a music file, corresponding to the HTML tag `<audio>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockaudio)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichBlockAudio {
    /// The audio
    pub audio: Audio,

    /// *Optional*. Caption of the block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen
