use serde::{Deserialize, Serialize};

/// A custom emoji.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#emojitypecustomemoji)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "custom_emoji", tag = "type")]
pub struct EmojiTypeCustomEmoji {
    /// Identifier of the custom emoji. Use [getCustomEmojiStickers](https://core.telegram.org/bots/api/#getcustomemojistickers) to get full information about the sticker.
    pub custom_emoji_id: String,
}

// Divider: all content below this line will be preserved after code regen
