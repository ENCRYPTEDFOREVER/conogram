use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A reference.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextreference)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "reference", tag = "type")]
pub struct RichTextReference {
    /// Text of the reference
    pub text: Box<RichText>,

    /// The name of the reference
    pub name: String,
}

// Divider: all content below this line will be preserved after code regen
