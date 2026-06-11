use serde::{Deserialize, Serialize};

use crate::entities::{rich_block::RichBlock, rich_block_caption::RichBlockCaption};

/// A slideshow, corresponding to the custom HTML tag `<tg-slideshow>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockslideshow)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichBlockSlideshow {
    /// Elements of the slideshow
    pub blocks: Vec<RichBlock>,

    /// *Optional*. Caption of the block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen
