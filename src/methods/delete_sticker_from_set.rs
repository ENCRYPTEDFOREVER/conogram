use conogram_derives::Request;
use serde::Serialize;

/// Use this method to delete a sticker from a set created by the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#deletestickerfromset)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct DeleteStickerFromSetParams {
    /// File identifier of the sticker
    pub sticker: String,
}

// Divider: all content below this line will be preserved after code regen
