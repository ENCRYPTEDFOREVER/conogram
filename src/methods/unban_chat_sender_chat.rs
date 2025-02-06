use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#unbanchatsenderchat)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct UnbanChatSenderChatParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}

// Divider: all content below this line will be preserved after code regen
