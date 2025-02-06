use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is [unbanned](https://core.telegram.org/bots/api/#unbanchatsenderchat), the owner of the banned chat won't be able to send messages on behalf of **any of their channels**. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#banchatsenderchat)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct BanChatSenderChatParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}

// Divider: all content below this line will be preserved after code regen
