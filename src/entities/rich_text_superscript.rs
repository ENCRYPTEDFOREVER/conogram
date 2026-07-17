use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A superscript text.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextsuperscript)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "superscript", tag = "type")]
pub struct RichTextSuperscript {
    /// The text
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
