use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A text covered by a spoiler.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextspoiler)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextSpoiler {
    /// The text
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
