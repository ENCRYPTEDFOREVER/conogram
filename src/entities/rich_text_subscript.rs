use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A subscript text.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextsubscript)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "subscript", tag = "type")]
pub struct RichTextSubscript {
    /// The text
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
