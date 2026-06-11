use serde::{Deserialize, Serialize};

use crate::{entities::rich_text::RichText, utils::deserialize_utils::is_false};

/// Cell in a table.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblocktablecell)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichBlockTableCell {
    /// *Optional*. Text in the cell. If omitted, then the cell is invisible.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<RichText>>,

    /// *Optional*. *True*, if the cell is a header cell
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_header: bool,

    /// *Optional*. The number of columns the cell spans if it is bigger than 1
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colspan: Option<i64>,

    /// *Optional*. The number of rows the cell spans if it is bigger than 1
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rowspan: Option<i64>,

    /// Horizontal cell content alignment. Currently, must be one of “left”, “center”, or “right”.
    pub align: RichBlockTableCellAlign,

    /// Vertical cell content alignment. Currently, must be one of “top”, “middle”, or “bottom”.
    pub valign: RichBlockTableCellValign,
}

/// Horizontal cell content alignment. Currently, must be one of “left”, “center”, or “right”.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RichBlockTableCellAlign {
    /// `left`
    #[default]
    #[serde(rename = "left")]
    Left,

    /// `center`
    #[serde(rename = "center")]
    Center,

    /// `right`
    #[serde(rename = "right")]
    Right,
}

/// Vertical cell content alignment. Currently, must be one of “top”, “middle”, or “bottom”.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RichBlockTableCellValign {
    /// `top`
    #[default]
    #[serde(rename = "top")]
    Top,

    /// `middle`
    #[serde(rename = "middle")]
    Middle,

    /// `bottom`
    #[serde(rename = "bottom")]
    Bottom,
}

// Divider: all content below this line will be preserved after code regen
