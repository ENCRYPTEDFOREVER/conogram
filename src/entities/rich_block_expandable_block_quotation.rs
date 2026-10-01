use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A block quotation, corresponding to the HTML tag `<blockquote>` with custom attribute `"expandable"`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockexpandableblockquotation)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "expandable_blockquote", tag = "type")]
pub struct RichBlockExpandableBlockQuotation {
    /// Content of the block
    pub text: Box<RichText>,

    /// *Optional*. Credit of the block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit: Option<Box<RichText>>,
}

// Divider: all content below this line will be preserved after code regen
