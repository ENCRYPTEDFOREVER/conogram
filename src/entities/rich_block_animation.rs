use serde::{Deserialize, Serialize};

use crate::{
    entities::{animation::Animation, rich_block_caption::RichBlockCaption},
    utils::deserialize_utils::is_false,
};

/// A block with an animation, corresponding to the HTML tag `<video>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockanimation)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "animation", tag = "type")]
pub struct RichBlockAnimation {
    /// The animation
    pub animation: Animation,

    /// *Optional*. *True*, if the media preview is covered by a spoiler animation
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_spoiler: bool,

    /// *Optional*. Caption of the block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen
