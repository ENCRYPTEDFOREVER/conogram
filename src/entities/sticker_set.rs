use crate::entities::photo_size::PhotoSize;
use crate::entities::sticker::Sticker;
use serde::{Deserialize, Serialize};

///This object represents a sticker set.
///API Reference: [link](https://core.telegram.org/bots/api/#stickerset)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StickerSet {
    ///Sticker set name
    pub name: String,

    ///Sticker set title
    pub title: String,

    ///Type of stickers in the set, currently one of “regular”, “mask”, “custom\_emoji”
    pub sticker_type: StickerSetStickerType,

    ///*True*, if the sticker set contains [animated stickers](https://telegram.org/blog/animated-stickers)
    pub is_animated: bool,

    ///*True*, if the sticker set contains [video stickers](https://telegram.org/blog/video-stickers-better-reactions)
    pub is_video: bool,

    ///List of all set stickers
    pub stickers: Vec<Sticker>,

    ///*Optional*. Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

///Type of stickers in the set, currently one of “regular”, “mask”, “custom\_emoji”
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "sticker_type")]
pub enum StickerSetStickerType {
    #[default]
    /// "regular"
    #[serde(rename = "regular")]
    Regular,

    /// "mask"
    #[serde(rename = "mask")]
    Mask,

    /// "custom_emoji"
    #[serde(rename = "custom_emoji")]
    CustomEmoji,
}
// Divider: all content below this line will be preserved after code regen
impl StickerSet {
    pub fn get_url(&self) -> String {
        format!("https://t.me/addstickers/{}", self.name)
    }
}
