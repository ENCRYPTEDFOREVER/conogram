use serde::{Deserialize, Serialize};

use crate::entities::{rich_block::RichBlock, rich_text::RichText};

/// A block quotation, corresponding to the HTML tag `<blockquote>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockblockquotation)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichBlockBlockQuotation {
    /// Content of the block
    pub blocks: Vec<RichBlock>,

    /// *Optional*. Credit of the block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit: Option<Box<RichText>>,
}

// Divider: all content below this line will be preserved after code regen
