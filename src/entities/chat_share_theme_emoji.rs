use serde::{Deserialize, Serialize};

use crate::entities::emoji_type::EmojiType;

/// The theme with emoji placed on a background.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatsharethemeemoji)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "emoji", tag = "type")]
pub struct ChatShareThemeEmoji {
    /// The emoji; 0-6 regular or custom emoji
    pub emojis: Vec<EmojiType>,

    /// *Optional*. Identifier of the backdrop used as the background; omitted if the default background is used. Use [getChatShareThemeBackdrops](https://core.telegram.org/bots/api/#getchatsharethemebackdrops) to get details about the backdrop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_backdrop_id: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
