use conogram_derives::Request;
use serde::Serialize;

/// Use this method to delete a sticker set that was created by the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#deletestickerset)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct DeleteStickerSetParams {
    /// Sticker set name
    pub name: String,
}

// Divider: all content below this line will be preserved after code regen
