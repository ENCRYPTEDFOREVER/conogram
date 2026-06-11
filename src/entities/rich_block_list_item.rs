use serde::{Deserialize, Serialize};

use crate::{entities::rich_block::RichBlock, utils::deserialize_utils::is_false};

/// An item of a list.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblocklistitem)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichBlockListItem {
    /// Label of the item
    pub label: String,

    /// The content of the item
    pub blocks: Vec<RichBlock>,

    /// *Optional*. *True*, if the item has a checkbox
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_checkbox: bool,

    /// *Optional*. *True*, if the item has a checked checkbox
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_checked: bool,

    /// *Optional*. For ordered lists, the numeric value of the item label
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,

    /// *Optional*. For ordered lists, the type of the item label; must be one of “a” for lowercase letters, “A” for uppercase letters, “i” for lowercase Roman numerals, “I” for uppercase Roman numerals, or “1” for decimal numbers
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<RichBlockListItemType>,
}

/// *Optional*. For ordered lists, the type of the item label; must be one of “a” for lowercase letters, “A” for uppercase letters, “i” for lowercase Roman numerals, “I” for uppercase Roman numerals, or “1” for decimal numbers
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RichBlockListItemType {
    /// `a`
    #[default]
    #[serde(rename = "a")]
    LowercaseLetter,

    /// `A`
    #[serde(rename = "A")]
    UppercaseLetter,

    /// `i`
    #[serde(rename = "i")]
    LowercaseNumeral,

    /// `I`
    #[serde(rename = "I")]
    UppercaseNumeral,

    /// `1`
    #[serde(rename = "1")]
    Decimal,
}

// Divider: all content below this line will be preserved after code regen
