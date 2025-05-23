use serde::{Deserialize, Serialize};

use crate::{
    entities::{file::File, mask_position::MaskPosition, photo_size::PhotoSize},
    utils::deserialize_utils::is_false,
};

/// This object represents a sticker.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#sticker)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,

    /// Type of the sticker, currently one of “regular”, “mask”, “custom\_emoji”. The type of the sticker is independent from its format, which is determined by the fields *is\_animated* and *is\_video*.
    #[serde(rename = "type")]
    pub type_: StickerType,

    /// Sticker width
    pub width: i64,

    /// Sticker height
    pub height: i64,

    /// *True*, if the sticker is [animated](https://telegram.org/blog/animated-stickers)
    pub is_animated: bool,

    /// *True*, if the sticker is a [video sticker](https://telegram.org/blog/video-stickers-better-reactions)
    pub is_video: bool,

    /// *Optional*. Sticker thumbnail in the .WEBP or .JPG format
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,

    /// *Optional*. Emoji associated with the sticker
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,

    /// *Optional*. Name of the sticker set to which the sticker belongs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,

    /// *Optional*. For premium regular stickers, premium animation for the sticker
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<File>,

    /// *Optional*. For mask stickers, the position where the mask should be placed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,

    /// *Optional*. For custom emoji stickers, unique identifier of the custom emoji
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,

    /// *Optional*. *True*, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
    #[serde(default, skip_serializing_if = "is_false")]
    pub needs_repainting: bool,

    /// *Optional*. File size in bytes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

/// Type of the sticker, currently one of “regular”, “mask”, “custom\_emoji”. The type of the sticker is independent from its format, which is determined by the fields *is\_animated* and *is\_video*.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum StickerType {
    /// `regular`
    #[default]
    #[serde(rename = "regular")]
    Regular,

    /// `mask`
    #[serde(rename = "mask")]
    Mask,

    /// `custom_emoji`
    #[serde(rename = "custom_emoji")]
    CustomEmoji,
}

// Divider: all content below this line will be preserved after code regen
use crate::{api::Api, methods::get_sticker_set::GetStickerSetRequest};

impl Sticker {
    /// Returns a [`GetStickerSetRequest`] if the sticker has a set, e.g. ([`self.set_name`](Self::set_name) is `Some`)
    pub fn get_sticker_set<'a>(&self, api: &'a Api) -> Option<GetStickerSetRequest<'a>> {
        self.set_name
            .as_ref()
            .map(|set_name| api.get_sticker_set(set_name))
    }
}
