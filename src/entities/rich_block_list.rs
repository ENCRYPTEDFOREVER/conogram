use serde::{Deserialize, Serialize};

use crate::entities::rich_block_list_item::RichBlockListItem;

/// A list of blocks, corresponding to the HTML tag `<ul>` or `<ol>` with multiple nested tags `<li>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblocklist)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "list", tag = "type")]
pub struct RichBlockList {
    /// Items of the list
    pub items: Vec<RichBlockListItem>,
}

// Divider: all content below this line will be preserved after code regen
