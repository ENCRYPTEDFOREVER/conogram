use serde::Serialize;

use crate::{
    entities::{rich_block_table_cell::RichBlockTableCell, rich_text::RichText},
    utils::deserialize_utils::is_false,
};

/// A table, corresponding to the HTML tag `<table>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblocktable)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "table", tag = "type")]
pub struct InputRichBlockTable {
    /// Cells of the table
    pub cells: Vec<Vec<RichBlockTableCell>>,

    /// *Optional*. Pass *True* if the table has borders
    #[serde(skip_serializing_if = "is_false")]
    pub is_bordered: bool,

    /// *Optional*. Pass *True* if the table is striped
    #[serde(skip_serializing_if = "is_false")]
    pub is_striped: bool,

    /// *Optional*. Caption of the table
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Box<RichText>>,
}

// Divider: all content below this line will be preserved after code regen
