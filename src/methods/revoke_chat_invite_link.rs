use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{chat_invite_link::ChatInviteLink, misc::chat_id::ChatId};

/// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#revokechatinvitelink)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = ChatInviteLink)]
pub struct RevokeChatInviteLinkParams {
    /// Unique identifier of the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// The invite link to revoke
    pub invite_link: String,
}

// Divider: all content below this line will be preserved after code regen
