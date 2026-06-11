use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// An italicized text.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextitalic)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextItalic {
    /// The text
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
