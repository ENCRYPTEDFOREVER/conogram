use serde::{Deserialize, Serialize};

use crate::{
    entities::{chat::Chat, chat_invite_link::ChatInviteLink, chat_member::ChatMember, user::User},
    utils::deserialize_utils::is_false,
};

/// This object represents changes in the status of a chat member.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatmemberupdated)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Box<Chat>,

    /// Performer of the action, which resulted in the change
    pub from: User,

    /// Date the change was done in Unix time
    pub date: i64,

    /// Previous information about the chat member
    pub old_chat_member: ChatMember,

    /// New information about the chat member
    pub new_chat_member: ChatMember,

    /// *Optional*. Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,

    /// *Optional*. True, if the user joined the chat after sending a direct join request without using an invite link and being approved by an administrator
    #[serde(default, skip_serializing_if = "is_false")]
    pub via_join_request: bool,

    /// *Optional*. True, if the user joined the chat via a chat folder invite link
    #[serde(default, skip_serializing_if = "is_false")]
    pub via_chat_folder_invite_link: bool,
}

// Divider: all content below this line will be preserved after code regen
