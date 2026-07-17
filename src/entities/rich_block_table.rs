use serde::{Deserialize, Serialize};

use crate::{
    entities::{rich_block_table_cell::RichBlockTableCell, rich_text::RichText},
    utils::deserialize_utils::is_false,
};

/// A table, corresponding to the HTML tag `<table>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblocktable)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "table", tag = "type")]
pub struct RichBlockTable {
    /// Cells of the table
    pub cells: Vec<Vec<RichBlockTableCell>>,

    /// *Optional*. *True*, if the table has borders
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_bordered: bool,

    /// *Optional*. *True*, if the table is striped
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_striped: bool,

    /// *Optional*. Caption of the table
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<Box<RichText>>,
}

// Divider: all content below this line will be preserved after code regen
