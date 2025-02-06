use conogram_derives::Request;
use serde::Serialize;

use crate::entities::sticker_set::StickerSet;

/// Use this method to get a sticker set. On success, a [StickerSet](https://core.telegram.org/bots/api/#stickerset) object is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getstickerset)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = StickerSet)]
pub struct GetStickerSetParams {
    /// Name of the sticker set
    pub name: String,
}

// Divider: all content below this line will be preserved after code regen
