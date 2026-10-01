use serde::Serialize;

use crate::entities::{
    rich_block_table_cell::RichBlockTableCellAlign, rich_message_button::RichMessageButton,
};

/// A block containing a list of buttons that are shown in one row, corresponding to the custom HTML tag `<tg-button-row>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockbuttons)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "buttons", tag = "type")]
pub struct InputRichBlockButtons {
    /// List of 1-8 buttons to send
    pub buttons: Vec<RichMessageButton>,

    /// *Optional*. Horizontal alignment of the buttons. Currently, must be one of “left”, “center”, or “right”.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<RichBlockTableCellAlign>,
}

// Divider: all content below this line will be preserved after code regen
