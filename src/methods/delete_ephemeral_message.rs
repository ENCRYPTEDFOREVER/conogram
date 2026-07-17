use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to delete an ephemeral message. Note that it is not guaranteed that the user will receive the message deletion event, especially if they are offline. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#deleteephemeralmessage)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct DeleteEphemeralMessageParams {
    /// Unique identifier for the target chat or username of the target supergroup in the format `@username`
    pub chat_id: ChatId,

    /// Identifier of the user who received the message
    pub receiver_user_id: i64,

    /// Identifier of the ephemeral message to delete
    pub ephemeral_message_id: i64,
}

// Divider: all content below this line will be preserved after code regen
