use conogram_derives::Request;
use serde::Serialize;

/// Transfers an owned unique gift to another user. Requires the *can\_transfer\_and\_upgrade\_gifts* business bot right. Requires *can\_transfer\_stars* business bot right if the transfer is paid. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#transfergift)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct TransferGiftParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// Unique identifier of the regular gift that should be transferred
    pub owned_gift_id: String,

    /// Unique identifier of the chat which will own the gift. The chat must be active in the last 24 hours.
    pub new_owner_chat_id: i64,

    /// The amount of Telegram Stars that will be paid for the transfer from the business account balance. If positive, then the *can\_transfer\_stars* business bot right is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_count: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
