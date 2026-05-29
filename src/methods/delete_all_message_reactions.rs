use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to remove up to 10000 recent reactions in a group or a supergroup chat added by a given user or chat. The bot must have the 'can\_delete\_messages' administrator right in the chat. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#deleteallmessagereactions)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct DeleteAllMessageReactionsParams {
    /// Unique identifier for the target chat or username of the target supergroup in the format `@username`
    pub chat_id: ChatId,

    /// Identifier of the user whose reactions will be removed, if the reactions were added by a user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// Identifier of the chat whose reactions will be removed, if the reactions were added by a chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat_id: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
