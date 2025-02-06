use conogram_derives::Request;
use serde::Serialize;

/// Use this method to set the thumbnail of a custom emoji sticker set. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setcustomemojistickersetthumbnail)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetCustomEmojiStickerSetThumbnailParams {
    /// Sticker set name
    pub name: String,

    /// Custom emoji identifier of a sticker from the sticker set; pass an empty string to drop the thumbnail and use the first sticker as the thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
