use serde::{Deserialize, Serialize};

/// A custom emoji.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextcustomemoji)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RichTextCustomEmoji {
    /// Unique identifier of the custom emoji. Use [getCustomEmojiStickers](https://core.telegram.org/bots/api/#getcustomemojistickers) to get full information about the sticker.
    pub custom_emoji_id: String,

    /// Alternative emoji for the custom emoji
    pub alternative_text: String,
}

// Divider: all content below this line will be preserved after code regen
