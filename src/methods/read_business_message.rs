use conogram_derives::Request;
use serde::Serialize;

/// Marks incoming message as read on behalf of a business account. Requires the *can\_read\_messages* business bot right. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#readbusinessmessage)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct ReadBusinessMessageParams {
    /// Unique identifier of the business connection on behalf of which to read the message
    pub business_connection_id: String,

    /// Unique identifier of the chat in which the message was received. The chat must have been active in the last 24 hours.
    pub chat_id: i64,

    /// Unique identifier of the message to mark as read
    pub message_id: i64,
}

// Divider: all content below this line will be preserved after code regen
