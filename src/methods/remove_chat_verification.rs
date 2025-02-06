use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Removes verification from a chat that is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#removechatverification)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct RemoveChatVerificationParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
}

// Divider: all content below this line will be preserved after code regen
