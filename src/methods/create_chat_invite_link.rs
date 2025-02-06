use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{chat_invite_link::ChatInviteLink, misc::chat_id::ChatId},
    utils::deserialize_utils::is_false,
};

/// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#createchatinvitelink)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = ChatInviteLink)]
pub struct CreateChatInviteLinkParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,

    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,

    /// *True*, if users joining the chat via the link need to be approved by chat administrators. If *True*, *member\_limit* can't be specified
    #[serde(skip_serializing_if = "is_false")]
    pub creates_join_request: bool,
}

// Divider: all content below this line will be preserved after code regen
