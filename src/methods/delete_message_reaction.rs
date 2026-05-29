use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to remove a reaction from a message in a group or a supergroup chat. The bot must have the 'can\_delete\_messages' administrator right in the chat. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#deletemessagereaction)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct DeleteMessageReactionParams {
    /// Unique identifier for the target chat or username of the target supergroup in the format `@username`
    pub chat_id: ChatId,

    /// Identifier of the target message
    pub message_id: i64,

    /// Identifier of the user whose reaction will be removed, if the reaction was added by a user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// Identifier of the chat whose reaction will be removed, if the reaction was added by a chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat_id: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
