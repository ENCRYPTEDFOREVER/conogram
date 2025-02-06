use conogram_derives::Request;
use serde::Serialize;

/// Use this method to move a sticker in a set created by the bot to a specific position. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setstickerpositioninset)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetStickerPositionInSetParams {
    /// File identifier of the sticker
    pub sticker: String,

    /// New sticker position in the set, zero-based
    pub position: i64,
}

// Divider: all content below this line will be preserved after code regen
