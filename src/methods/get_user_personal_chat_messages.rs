use conogram_derives::Request;
use serde::Serialize;

use crate::entities::message::Message;

/// Use this method to get the last messages from the personal chat (i.e., the chat currently added to their profile) of a given user. On success, an Array of [Message](https://core.telegram.org/bots/api/#message) objects is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getuserpersonalchatmessages)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Vec<Message>)]
pub struct GetUserPersonalChatMessagesParams {
    /// Unique identifier for the target user
    pub user_id: i64,

    /// The maximum number of messages to return; 1-20
    pub limit: i64,
}

// Divider: all content below this line will be preserved after code regen
