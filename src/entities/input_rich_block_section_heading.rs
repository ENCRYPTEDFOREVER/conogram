use serde::Serialize;

use crate::entities::rich_text::RichText;

/// A section heading, corresponding to the HTML tags `<h1>`, `<h2>`, `<h3>`, `<h4>`, `<h5>`, or `<h6>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblocksectionheading)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "heading", tag = "type")]
pub struct InputRichBlockSectionHeading {
    /// Text of the block
    pub text: Box<RichText>,

    /// Relative size of the text font; 1-6, 1 is the largest, 6 is the smallest
    pub size: i64,
}

// Divider: all content below this line will be preserved after code regen
