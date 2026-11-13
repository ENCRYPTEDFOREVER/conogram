use serde::{Deserialize, Serialize};

use crate::entities::{
    emoji_type_custom_emoji::EmojiTypeCustomEmoji, emoji_type_emoji::EmojiTypeEmoji,
};

/// This object describes an emoji. Currently, it can be one of
///
/// * [EmojiTypeEmoji](https://core.telegram.org/bots/api/#emojitypeemoji)
/// * [EmojiTypeCustomEmoji](https://core.telegram.org/bots/api/#emojitypecustomemoji)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#emojitype)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EmojiType {
    /// A regular emoji.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#emojitypeemoji)
    #[serde(rename = "emoji")]
    Emoji(EmojiTypeEmoji),

    /// A custom emoji.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#emojitypecustomemoji)
    #[serde(rename = "custom_emoji")]
    CustomEmoji(EmojiTypeCustomEmoji),
}

impl Default for EmojiType {
    fn default() -> Self {
        Self::Emoji(EmojiTypeEmoji::default())
    }
}

impl From<EmojiTypeEmoji> for EmojiType {
    fn from(value: EmojiTypeEmoji) -> Self {
        Self::Emoji(value)
    }
}

impl From<EmojiTypeCustomEmoji> for EmojiType {
    fn from(value: EmojiTypeCustomEmoji) -> Self {
        Self::CustomEmoji(value)
    }
}

// Divider: all content below this line will be preserved after code regen
