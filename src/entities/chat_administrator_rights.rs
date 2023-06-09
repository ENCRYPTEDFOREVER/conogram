use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///Represents the rights of an administrator in a chat.
///API Reference: [link](https://core.telegram.org/bots/api/#chatadministratorrights)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatAdministratorRights {
    ///*True*, if the user's presence in the chat is hidden
    pub is_anonymous: bool,

    ///*True*, if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    pub can_manage_chat: bool,

    ///*True*, if the administrator can delete messages of other users
    pub can_delete_messages: bool,

    ///*True*, if the administrator can manage video chats
    pub can_manage_video_chats: bool,

    ///*True*, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: bool,

    ///*True*, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,

    ///*True*, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,

    ///*True*, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,

    ///*Optional*. *True*, if the administrator can post in the channel; channels only
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_post_messages: bool,

    ///*Optional*. *True*, if the administrator can edit messages of other users and can pin messages; channels only
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_edit_messages: bool,

    ///*Optional*. *True*, if the user is allowed to pin messages; groups and supergroups only
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_pin_messages: bool,

    ///*Optional*. *True*, if the user is allowed to create, rename, close, and reopen forum topics; supergroups only
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_manage_topics: bool,
}
// Divider: all content below this line will be preserved after code regen
