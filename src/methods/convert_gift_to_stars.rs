use conogram_derives::Request;
use serde::Serialize;

/// Converts a given regular gift to Telegram Stars. Requires the *can\_convert\_gifts\_to\_stars* business bot right. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#convertgifttostars)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct ConvertGiftToStarsParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// Unique identifier of the regular gift that should be converted to Telegram Stars
    pub owned_gift_id: String,
}

// Divider: all content below this line will be preserved after code regen
