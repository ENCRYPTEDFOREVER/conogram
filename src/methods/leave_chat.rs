use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method for your bot to leave a group, supergroup or channel. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#leavechat)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct LeaveChatParams {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
}

// Divider: all content below this line will be preserved after code regen
