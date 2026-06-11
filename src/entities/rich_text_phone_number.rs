use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A text with a phone number.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextphonenumber)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextPhoneNumber {
    /// The text
    pub text: Box<RichText>,

    /// The phone number
    pub phone_number: String,
}

// Divider: all content below this line will be preserved after code regen
