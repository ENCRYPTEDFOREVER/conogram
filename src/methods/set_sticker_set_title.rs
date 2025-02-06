use conogram_derives::Request;
use serde::Serialize;

/// Use this method to set the title of a created sticker set. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setstickersettitle)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetStickerSetTitleParams {
    /// Sticker set name
    pub name: String,

    /// Sticker set title, 1-64 characters
    pub title: String,
}

// Divider: all content below this line will be preserved after code regen
