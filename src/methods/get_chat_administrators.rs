use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{chat_member::ChatMember, misc::chat_id::ChatId},
    utils::deserialize_utils::is_false,
};

/// Use this method to get a list of administrators in a chat. Returns an Array of [ChatMember](https://core.telegram.org/bots/api/#chatmember) objects.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getchatadministrators)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Vec<ChatMember>)]
pub struct GetChatAdministratorsParams {
    /// Unique identifier for the target chat or username of the target supergroup or channel in the format `@username`
    pub chat_id: ChatId,

    /// Pass *True* to additionally receive all bots that are administrators of the chat. By default, bots other than the current bot are omitted.
    #[serde(skip_serializing_if = "is_false")]
    pub return_bots: bool,
}

// Divider: all content below this line will be preserved after code regen
