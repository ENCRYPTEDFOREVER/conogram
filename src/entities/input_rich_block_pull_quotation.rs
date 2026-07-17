use serde::Serialize;

use crate::entities::rich_text::RichText;

/// A quotation with centered text, loosely corresponding to the HTML tag `<aside>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockpullquotation)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "pullquote", tag = "type")]
pub struct InputRichBlockPullQuotation {
    /// Text of the block
    pub text: Box<RichText>,

    /// *Optional*. Credit of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<Box<RichText>>,
}

// Divider: all content below this line will be preserved after code regen
