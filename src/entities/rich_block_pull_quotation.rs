use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A quotation with centered text, loosely corresponding to the HTML tag `<aside>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockpullquotation)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "pullquote", tag = "type")]
pub struct RichBlockPullQuotation {
    /// Text of the block
    pub text: Box<RichText>,

    /// *Optional*. Credit of the block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit: Option<Box<RichText>>,
}

// Divider: all content below this line will be preserved after code regen
