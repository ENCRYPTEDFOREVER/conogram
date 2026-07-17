use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A text with a bank card number.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextbankcardnumber)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "bank_card_number", tag = "type")]
pub struct RichTextBankCardNumber {
    /// The text
    pub text: Box<RichText>,

    /// The bank card number
    pub bank_card_number: String,
}

// Divider: all content below this line will be preserved after code regen
