use serde::{Deserialize, Serialize};

/// A regular emoji.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#emojitypeemoji)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "emoji", tag = "type")]
pub struct EmojiTypeEmoji {
    /// The emoji
    pub emoji: String,
}

// Divider: all content below this line will be preserved after code regen
