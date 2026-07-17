use serde::Serialize;

use crate::entities::rich_text::RichText;

/// A footer, corresponding to the HTML tag `<footer>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockfooter)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "footer", tag = "type")]
pub struct InputRichBlockFooter {
    /// Text of the block
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
