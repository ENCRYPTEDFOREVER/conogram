use conogram_derives::Request;
use serde::Serialize;

use crate::entities::input_sticker::InputSticker;

/// Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#addstickertoset)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct AddStickerToSetParams {
    /// User identifier of sticker set owner
    pub user_id: i64,

    /// Sticker set name
    pub name: String,

    /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed.
    pub sticker: InputSticker,
}

// Divider: all content below this line will be preserved after code regen
