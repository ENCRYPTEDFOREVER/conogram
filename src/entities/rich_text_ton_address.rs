use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// An address in the TON blockchain.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtexttonaddress)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ton_address", tag = "type")]
pub struct RichTextTonAddress {
    /// Text of the address
    pub text: Box<RichText>,

    /// The address
    pub address: String,
}

// Divider: all content below this line will be preserved after code regen
