use conogram_derives::Request;
use serde::Serialize;

use crate::entities::input_sticker::InputSticker;

/// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling [deleteStickerFromSet](https://core.telegram.org/bots/api/#deletestickerfromset), then [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), then [setStickerPositionInSet](https://core.telegram.org/bots/api/#setstickerpositioninset). Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#replacestickerinset)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct ReplaceStickerInSetParams {
    /// User identifier of the sticker set owner
    pub user_id: i64,

    /// Sticker set name
    pub name: String,

    /// File identifier of the replaced sticker
    pub old_sticker: String,

    /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set remains unchanged.
    pub sticker: InputSticker,
}

// Divider: all content below this line will be preserved after code regen
