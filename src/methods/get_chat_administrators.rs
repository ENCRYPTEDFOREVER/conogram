use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{chat_member::ChatMember, misc::chat_id::ChatId};

/// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of [ChatMember](https://core.telegram.org/bots/api/#chatmember) objects.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getchatadministrators)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Vec<ChatMember>)]
pub struct GetChatAdministratorsParams {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
}

// Divider: all content below this line will be preserved after code regen
