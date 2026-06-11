use serde::{Deserialize, Serialize};

use crate::{
    entities::{rich_block::RichBlock, rich_text::RichText},
    utils::deserialize_utils::is_false,
};

/// An expandable block for details disclosure, corresponding to the HTML tag `<details>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockdetails)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichBlockDetails {
    /// Always shown summary of the block
    pub summary: Box<RichText>,

    /// Content of the block
    pub blocks: Vec<RichBlock>,

    /// *Optional*. *True*, if the content of the block is visible by default
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_open: bool,
}

// Divider: all content below this line will be preserved after code regen
