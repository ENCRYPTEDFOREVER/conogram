use conogram_derives::Request;
use serde::Serialize;

use crate::{entities::misc::chat_id::ChatId, utils::deserialize_utils::is_false};

/// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass *False* for all boolean parameters to demote a user. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#promotechatmember)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct PromoteChatMemberParams {
    /// Unique identifier for the target chat or username of the target channel in the format `@username`
    pub chat_id: ChatId,

    /// Unique identifier of the target user
    pub user_id: i64,

    /// Pass *True* if the administrator's presence in the chat is hidden
    #[serde(skip_serializing_if = "is_false")]
    pub is_anonymous: bool,

    /// Pass *True* if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    #[serde(skip_serializing_if = "is_false")]
    pub can_manage_chat: bool,

    /// Pass *True* if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "is_false")]
    pub can_delete_messages: bool,

    /// Pass *True* if the administrator can manage video chats
    #[serde(skip_serializing_if = "is_false")]
    pub can_manage_video_chats: bool,

    /// Pass *True* if the administrator can restrict, ban or unban chat members, or access supergroup statistics. For backward compatibility, defaults to *True* for promotions of channel administrators.
    #[serde(skip_serializing_if = "is_false")]
    pub can_restrict_members: bool,

    /// Pass *True* if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[serde(skip_serializing_if = "is_false")]
    pub can_promote_members: bool,

    /// Pass *True* if the administrator can change chat title, photo and other settings
    #[serde(skip_serializing_if = "is_false")]
    pub can_change_info: bool,

    /// Pass *True* if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "is_false")]
    pub can_invite_users: bool,

    /// Pass *True* if the administrator can post stories to the chat
    #[serde(skip_serializing_if = "is_false")]
    pub can_post_stories: bool,

    /// Pass *True* if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    #[serde(skip_serializing_if = "is_false")]
    pub can_edit_stories: bool,

    /// Pass *True* if the administrator can delete stories posted by other users
    #[serde(skip_serializing_if = "is_false")]
    pub can_delete_stories: bool,

    /// Pass *True* if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    #[serde(skip_serializing_if = "is_false")]
    pub can_post_messages: bool,

    /// Pass *True* if the administrator can edit messages of other users and can pin messages; for channels only
    #[serde(skip_serializing_if = "is_false")]
    pub can_edit_messages: bool,

    /// Pass *True* if the administrator can pin messages; for supergroups only
    #[serde(skip_serializing_if = "is_false")]
    pub can_pin_messages: bool,

    /// Pass *True* if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    #[serde(skip_serializing_if = "is_false")]
    pub can_manage_topics: bool,

    /// Pass *True* if the administrator can manage direct messages within the channel and decline suggested posts; for channels only
    #[serde(skip_serializing_if = "is_false")]
    pub can_manage_direct_messages: bool,

    /// Pass *True* if the administrator can edit the tags of regular members; for groups and supergroups only
    #[serde(skip_serializing_if = "is_false")]
    pub can_manage_tags: bool,
}

// Divider: all content below this line will be preserved after code regen
