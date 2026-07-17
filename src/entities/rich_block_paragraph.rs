use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A text paragraph, corresponding to the HTML tag `<p>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockparagraph)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "paragraph", tag = "type")]
pub struct RichBlockParagraph {
    /// Text of the block
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
