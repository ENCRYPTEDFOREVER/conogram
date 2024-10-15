use serde::{Deserialize, Serialize};

use crate::entities::{
    photo_size::PhotoSize,
    sticker::{Sticker, StickerType},
};

/// This object represents a sticker set.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#stickerset)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,

    /// Sticker set title
    pub title: String,

    /// Type of stickers in the set, currently one of “regular”, “mask”, “custom\_emoji”
    pub sticker_type: StickerType,

    /// List of all set stickers
    pub stickers: Vec<Sticker>,

    /// *Optional*. Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

// Divider: all content below this line will be preserved after code regen
impl StickerSet {
    pub fn get_url(&self) -> String {
        format!("https://t.me/addstickers/{}", self.name)
    }
}
