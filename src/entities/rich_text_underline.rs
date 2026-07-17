use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// An underlined text.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextunderline)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "underline", tag = "type")]
pub struct RichTextUnderline {
    /// The text
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
