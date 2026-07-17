use serde::Serialize;

use crate::entities::rich_text::RichText;

/// A text paragraph, corresponding to the HTML tag `<p>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockparagraph)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "paragraph", tag = "type")]
pub struct InputRichBlockParagraph {
    /// Text of the block
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
