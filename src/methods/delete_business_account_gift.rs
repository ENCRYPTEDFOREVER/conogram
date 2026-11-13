use conogram_derives::Request;
use serde::Serialize;

/// Deletes a regular gift owned by a managed business account without receiving Telegram Stars for it. Requires the *can\_convert\_gifts\_to\_stars* business bot right. The gift can be deleted only if it isn't limited (the field *total\_count* of the [Gift](https://core.telegram.org/bots/api/#gift) is omitted) and wasn't refunded. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#deletebusinessaccountgift)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct DeleteBusinessAccountGiftParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// Unique identifier of the regular gift that should be deleted
    pub owned_gift_id: String,
}

// Divider: all content below this line will be preserved after code regen
