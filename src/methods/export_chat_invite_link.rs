use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as *String* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#exportchatinvitelink)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = String)]
pub struct ExportChatInviteLinkParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
}

// Divider: all content below this line will be preserved after code regen
