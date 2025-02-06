use conogram_derives::Request;
use serde::Serialize;

/// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setstickerkeywords)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetStickerKeywordsParams {
    /// File identifier of the sticker
    pub sticker: String,

    /// A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
}

// Divider: all content below this line will be preserved after code regen
