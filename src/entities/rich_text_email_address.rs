use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A text with an email address.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextemailaddress)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "email_address", tag = "type")]
pub struct RichTextEmailAddress {
    /// The text
    pub text: Box<RichText>,

    /// The email address
    pub email_address: String,
}

// Divider: all content below this line will be preserved after code regen
