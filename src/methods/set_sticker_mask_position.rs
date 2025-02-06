use conogram_derives::Request;
use serde::Serialize;

use crate::entities::mask_position::MaskPosition;

/// Use this method to change the [mask position](https://core.telegram.org/bots/api/#maskposition) of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setstickermaskposition)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetStickerMaskPositionParams {
    /// File identifier of the sticker
    pub sticker: String,

    /// A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

// Divider: all content below this line will be preserved after code regen
