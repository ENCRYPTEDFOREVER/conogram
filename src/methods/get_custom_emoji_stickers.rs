use conogram_derives::Request;
use serde::Serialize;

use crate::entities::sticker::Sticker;

/// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getcustomemojistickers)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Vec<Sticker>)]
pub struct GetCustomEmojiStickersParams {
    /// A JSON-serialized list of custom emoji identifiers. At most 200 custom emoji identifiers can be specified.
    pub custom_emoji_ids: Vec<String>,
}

// Divider: all content below this line will be preserved after code regen
