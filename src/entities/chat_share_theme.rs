use serde::{Deserialize, Serialize};

use crate::entities::{
    chat_share_theme_emoji::ChatShareThemeEmoji,
    chat_share_theme_unique_gift::ChatShareThemeUniqueGift,
};

/// This object describes the theme of the screen for sharing a link to a user or a chat. Currently, it can be one of
///
/// * [ChatShareThemeEmoji](https://core.telegram.org/bots/api/#chatsharethemeemoji)
/// * [ChatShareThemeUniqueGift](https://core.telegram.org/bots/api/#chatsharethemeuniquegift)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatsharetheme)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ChatShareTheme {
    /// The theme with emoji placed on a background.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#chatsharethemeemoji)
    #[serde(rename = "emoji")]
    Emoji(ChatShareThemeEmoji),

    /// The theme based on a unique gift owned by the user or the chat.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#chatsharethemeuniquegift)
    #[serde(rename = "unique_gift")]
    UniqueGift(ChatShareThemeUniqueGift),
}

impl Default for ChatShareTheme {
    fn default() -> Self {
        Self::Emoji(ChatShareThemeEmoji::default())
    }
}

impl From<ChatShareThemeEmoji> for ChatShareTheme {
    fn from(value: ChatShareThemeEmoji) -> Self {
        Self::Emoji(value)
    }
}

impl From<ChatShareThemeUniqueGift> for ChatShareTheme {
    fn from(value: ChatShareThemeUniqueGift) -> Self {
        Self::UniqueGift(value)
    }
}

// Divider: all content below this line will be preserved after code regen
