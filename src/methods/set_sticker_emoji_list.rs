use conogram_derives::Request;
use serde::Serialize;

/// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setstickeremojilist)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetStickerEmojiListParams {
    /// File identifier of the sticker
    pub sticker: String,

    /// A JSON-serialized list of 1-20 emoji associated with the sticker
    pub emoji_list: Vec<String>,
}

// Divider: all content below this line will be preserved after code regen
