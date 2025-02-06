use conogram_derives::Request;
use serde::Serialize;

use crate::{entities::input_sticker::InputSticker, utils::deserialize_utils::is_false};

/// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#createnewstickerset)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct CreateNewStickerSetParams {
    /// User identifier of created sticker set owner
    pub user_id: i64,

    /// Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g., *animals*). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in `"_by_<bot_username>"`. `<bot_username>` is case insensitive. 1-64 characters.
    pub name: String,

    /// Sticker set title, 1-64 characters
    pub title: String,

    /// A JSON-serialized list of 1-50 initial stickers to be added to the sticker set
    pub stickers: Vec<InputSticker>,

    /// Type of stickers in the set, pass “regular”, “mask”, or “custom\_emoji”. By default, a regular sticker set is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<StickerType>,

    /// Pass *True* if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only
    #[serde(skip_serializing_if = "is_false")]
    pub needs_repainting: bool,
}

/// Type of stickers in the set, pass “regular”, “mask”, or “custom\_emoji”. By default, a regular sticker set is created.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub enum StickerType {
    /// `mask`
    #[default]
    #[serde(rename = "mask")]
    Mask,

    /// `custom_emoji`
    #[serde(rename = "custom_emoji")]
    CustomEmoji,
}

// Divider: all content below this line will be preserved after code regen
