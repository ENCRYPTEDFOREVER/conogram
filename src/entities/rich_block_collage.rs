use serde::{Deserialize, Serialize};

use crate::entities::{rich_block::RichBlock, rich_block_caption::RichBlockCaption};

/// A collage, corresponding to the custom HTML tag `<tg-collage>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockcollage)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "collage", tag = "type")]
pub struct RichBlockCollage {
    /// Elements of the collage
    pub blocks: Vec<RichBlock>,

    /// *Optional*. Caption of the block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen
