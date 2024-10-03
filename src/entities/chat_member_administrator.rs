use serde::{Deserialize, Serialize};

use crate::{entities::user::User, utils::deserialize_utils::is_false};

/// Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that has some additional privileges.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatmemberadministrator)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberAdministrator {
    /// Information about the user
    pub user: User,

    /// *True*, if the bot is allowed to edit administrator privileges of that user
    pub can_be_edited: bool,

    /// *True*, if the user's presence in the chat is hidden
    pub is_anonymous: bool,

    /// *True*, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages and ignore slow mode. Implied by any other administrator privilege.
    pub can_manage_chat: bool,

    /// *True*, if the administrator can delete messages of other users
    pub can_delete_messages: bool,

    /// *True*, if the administrator can manage video chats
    pub can_manage_video_chats: bool,

    /// *True*, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    pub can_restrict_members: bool,

    /// *True*, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,

    /// *True*, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,

    /// *True*, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,

    /// *True*, if the administrator can post stories to the chat
    pub can_post_stories: bool,

    /// *True*, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    pub can_edit_stories: bool,

    /// *True*, if the administrator can delete stories posted by other users
    pub can_delete_stories: bool,

    /// *Optional*. *True*, if the administrator can post messages in the channel, or access channel statistics; for channels only
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_post_messages: bool,

    /// *Optional*. *True*, if the administrator can edit messages of other users and can pin messages; for channels only
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_edit_messages: bool,

    /// *Optional*. *True*, if the user is allowed to pin messages; for groups and supergroups only
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_pin_messages: bool,

    /// *Optional*. *True*, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_manage_topics: bool,

    /// *Optional*. Custom title for this user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
