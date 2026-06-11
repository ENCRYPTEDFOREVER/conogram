use serde::{Deserialize, Serialize};

use crate::entities::{rich_text::RichText, user::User};

/// A mention of a Telegram user by their identifier.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtexttextmention)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextTextMention {
    /// The text
    pub text: Box<RichText>,

    /// The mentioned user
    pub user: User,
}

// Divider: all content below this line will be preserved after code regen
