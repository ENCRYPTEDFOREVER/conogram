use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A mention by a username.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextmention)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextMention {
    /// The text
    pub text: Box<RichText>,

    /// The username
    pub username: String,
}

// Divider: all content below this line will be preserved after code regen
