use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///Represents the rights of an administrator in a chat.
///API Reference: [link](https://core.telegram.org/bots/api/#chatadministratorrights)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatAdministratorRights {
    ///*True*, if the user's presence in the chat is hidden
    pub is_anonymous: bool,

    ///*True*, if the administrator can access the chat event log, boost list in channels, see channel members, report spam messages, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    pub can_manage_chat: bool,

    ///*True*, if the administrator can delete messages of other users
    pub can_delete_messages: bool,

    ///*True*, if the administrator can manage video chats
    pub can_manage_video_chats: bool,

    ///*True*, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    pub can_restrict_members: bool,

    ///*True*, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,

    ///*True*, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,

    ///*True*, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,

    ///*Optional*. *True*, if the administrator can post messages in the channel, or access channel statistics; channels only
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_post_messages: bool,

    ///*Optional*. *True*, if the administrator can edit messages of other users and can pin messages; channels only
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_edit_messages: bool,

    ///*Optional*. *True*, if the user is allowed to pin messages; groups and supergroups only
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_pin_messages: bool,

    ///*Optional*. *True*, if the administrator can post stories in the channel; channels only
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_post_stories: bool,

    ///*Optional*. *True*, if the administrator can edit stories posted by other users; channels only
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_edit_stories: bool,

    ///*Optional*. *True*, if the administrator can delete stories posted by other users; channels only
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_delete_stories: bool,

    ///*Optional*. *True*, if the user is allowed to create, rename, close, and reopen forum topics; supergroups only
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_manage_topics: bool,
}
// Divider: all content below this line will be preserved after code regen
