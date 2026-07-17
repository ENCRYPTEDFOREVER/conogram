use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A strikethrough text.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextstrikethrough)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "strikethrough", tag = "type")]
pub struct RichTextStrikethrough {
    /// The text
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
