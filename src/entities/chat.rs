use crate::entities::chat_location::ChatLocation;
use crate::entities::chat_permissions::ChatPermissions;
use crate::entities::chat_photo::ChatPhoto;
use crate::entities::message::Message;
use crate::utils::deserialize_utils::deserialize_boxed_option;
use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///This object represents a chat.
///API Reference: [link](https://core.telegram.org/bots/api/#chat)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    ///Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,

    ///Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub type_: ChatType,

    ///*Optional*. Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    ///*Optional*. Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    ///*Optional*. First name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    ///*Optional*. Last name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    ///*Optional*. *True*, if the supergroup chat is a forum (has [topics](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_forum: bool,

    ///*Optional*. Chat photo. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,

    ///*Optional*. If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames#collectible-usernames); for private chats, supergroups and channels. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(default)]
    pub active_usernames: Vec<String>,

    ///*Optional*. Custom emoji identifier of emoji status of the other party in a private chat. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,

    ///*Optional*. Bio of the other party in a private chat. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    ///*Optional*. *True*, if privacy settings of the other party in the private chat allows to use `tg://user?id=<user_id>` links only in chats with the user. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_private_forwards: bool,

    ///*Optional*. *True*, if the privacy settings of the other party restrict sending voice and video note messages in the private chat. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_restricted_voice_and_video_messages: bool,

    ///*Optional*. *True*, if users need to join the supergroup before they can send messages. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub join_to_send_messages: bool,

    ///*Optional*. *True*, if all users directly joining the supergroup need to be approved by supergroup administrators. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub join_by_request: bool,

    ///*Optional*. Description, for groups, supergroups and channel chats. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    ///*Optional*. Primary invite link, for groups, supergroups and channel chats. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,

    ///*Optional*. The most recent pinned message (by sending date). Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(
        deserialize_with = "deserialize_boxed_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub pinned_message: Option<Box<Message>>,

    ///*Optional*. Default chat member permissions, for groups and supergroups. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,

    ///*Optional*. For supergroups, the minimum allowed delay between consecutive messages sent by each unpriviledged user; in seconds. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<i64>,

    ///*Optional*. The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i64>,

    ///*Optional*. *True*, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_aggressive_anti_spam_enabled: bool,

    ///*Optional*. *True*, if non-administrators can only get the list of bots and administrators in the chat. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_hidden_members: bool,

    ///*Optional*. *True*, if messages from the chat can't be forwarded to other chats. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_protected_content: bool,

    ///*Optional*. For supergroups, name of group sticker set. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,

    ///*Optional*. *True*, if the bot can change the group sticker set. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_set_sticker_set: bool,

    ///*Optional*. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,

    ///*Optional*. For supergroups, the location to which the supergroup is connected. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
}

///Type of chat, can be either “private”, “group”, “supergroup” or “channel”
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "type")]
pub enum ChatType {
    #[default]
    /// "private"
    #[serde(rename = "private")]
    Private,

    /// "group"
    #[serde(rename = "group")]
    Group,

    /// "supergroup"
    #[serde(rename = "supergroup")]
    Supergroup,

    /// "channel"
    #[serde(rename = "channel")]
    Channel,
}
// Divider: all content below this line will be preserved after code regen
use crate::api::API;

use super::misc::input_file::InputFile;

impl Chat {
    pub async fn unpin_all_messages<'a>(
        &'a self,
        api: &'a API,
    ) -> crate::methods::unpin_all_chat_messages::UnpinAllChatMessagesRequest {
        api.unpin_all_chat_messages(self.id)
    }

    pub async fn get_member_count<'a>(
        &'a self,
        api: &'a API,
    ) -> crate::methods::get_chat_member_count::GetChatMemberCountRequest {
        api.get_chat_member_count(self.id)
    }

    pub async fn get_administrators<'a>(
        &'a self,
        api: &'a API,
    ) -> crate::methods::get_chat_administrators::GetChatAdministratorsRequest {
        api.get_chat_administrators(self.id)
    }

    pub async fn get_member<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
    ) -> crate::methods::get_chat_member::GetChatMemberRequest {
        api.get_chat_member(self.id, user_id)
    }

    pub async fn update<'a>(&'a self, api: &'a API) -> crate::methods::get_chat::GetChatRequest {
        api.get_chat(self.id)
    }

    pub async fn ban_member<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
    ) -> crate::methods::ban_chat_member::BanChatMemberRequest {
        api.ban_chat_member(self.id, user_id)
    }

    pub async fn unban_member<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
    ) -> crate::methods::unban_chat_member::UnbanChatMemberRequest {
        api.unban_chat_member(self.id, user_id)
    }

    pub async fn ban_sender_chat<'a>(
        &'a self,
        api: &'a API,
        sender_chat_id: impl Into<i64>,
    ) -> crate::methods::ban_chat_sender_chat::BanChatSenderChatRequest {
        api.ban_chat_sender_chat(self.id, sender_chat_id)
    }

    pub async fn unban_sender_chat<'a>(
        &'a self,
        api: &'a API,
        sender_chat_id: impl Into<i64>,
    ) -> crate::methods::unban_chat_sender_chat::UnbanChatSenderChatRequest {
        api.unban_chat_sender_chat(self.id, sender_chat_id)
    }

    pub async fn set_administrator_custom_title<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
        custom_title: impl Into<String>,
    ) -> crate::methods::set_chat_administrator_custom_title::SetChatAdministratorCustomTitleRequest
    {
        api.set_chat_administrator_custom_title(self.id, user_id, custom_title)
    }

    pub async fn set_title<'a>(
        &'a self,
        api: &'a API,
        title: impl Into<String>,
    ) -> crate::methods::set_chat_title::SetChatTitleRequest {
        api.set_chat_title(self.id, title)
    }

    pub async fn set_description<'a>(
        &'a self,
        api: &'a API,
        description: Option<impl Into<String>>,
    ) -> crate::methods::set_chat_description::SetChatDescriptionRequest {
        if let Some(description) = description {
            api.set_chat_description(self.id).description(description)
        } else {
            api.set_chat_description(self.id)
        }
    }

    pub async fn set_permissions<'a>(
        &'a self,
        api: &'a API,
        permissions: impl Into<ChatPermissions>,
    ) -> crate::methods::set_chat_permissions::SetChatPermissionsRequest {
        api.set_chat_permissions(self.id, permissions)
    }

    pub async fn set_photo<'a>(
        &'a self,
        api: &'a API,
        photo: impl Into<InputFile>,
    ) -> crate::methods::set_chat_photo::SetChatPhotoRequest {
        api.set_chat_photo(self.id, photo)
    }

    pub async fn set_sticker_set<'a>(
        &'a self,
        api: &'a API,
        sticker_set_name: impl Into<String>,
    ) -> crate::methods::set_chat_sticker_set::SetChatStickerSetRequest {
        api.set_chat_sticker_set(self.id, sticker_set_name)
    }

    pub fn send_action<'a>(
        &'a self,
        api: &'a API,
        action: impl Into<crate::methods::send_chat_action::SendChatActionAction>,
    ) -> crate::methods::send_chat_action::SendChatActionRequest {
        api.send_chat_action(self.id, action)
    }

    pub async fn edit_invite_link<'a>(
        &'a self,
        api: &'a API,
        invite_link: impl Into<String>,
    ) -> crate::methods::edit_chat_invite_link::EditChatInviteLinkRequest {
        api.edit_chat_invite_link(self.id, invite_link)
    }

    pub async fn leave<'a>(&'a self, api: &'a API) -> crate::methods::leave_chat::LeaveChatRequest {
        api.leave_chat(self.id)
    }

    pub async fn delete_photo<'a>(
        &'a self,
        api: &'a API,
    ) -> crate::methods::delete_chat_photo::DeleteChatPhotoRequest {
        api.delete_chat_photo(self.id)
    }

    pub async fn delete_sticker_set<'a>(
        &'a self,
        api: &'a API,
    ) -> crate::methods::delete_chat_sticker_set::DeleteChatStickerSetRequest {
        api.delete_chat_sticker_set(self.id)
    }

    pub async fn export_invite_link<'a>(
        &'a self,
        api: &'a API,
    ) -> crate::methods::export_chat_invite_link::ExportChatInviteLinkRequest {
        api.export_chat_invite_link(self.id)
    }

    pub async fn revoke_invite_link<'a>(
        &'a self,
        api: &'a API,
        invite_link: impl Into<String>,
    ) -> crate::methods::revoke_chat_invite_link::RevokeChatInviteLinkRequest {
        api.revoke_chat_invite_link(self.id, invite_link)
    }

    pub async fn approve_join_request<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
    ) -> crate::methods::approve_chat_join_request::ApproveChatJoinRequestRequest {
        api.approve_chat_join_request(self.id, user_id)
    }

    pub async fn decline_join_request<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
    ) -> crate::methods::decline_chat_join_request::DeclineChatJoinRequestRequest {
        api.decline_chat_join_request(self.id, user_id)
    }

    pub async fn create_invite_link<'a>(
        &'a self,
        api: &'a API,
    ) -> crate::methods::create_chat_invite_link::CreateChatInviteLinkRequest {
        api.create_chat_invite_link(self.id)
    }

    pub async fn promote_member<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
    ) -> crate::methods::promote_chat_member::PromoteChatMemberRequest {
        api.promote_chat_member(self.id, user_id)
    }

    pub async fn restrict_member<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
        permissions: impl Into<ChatPermissions>,
    ) -> crate::methods::restrict_chat_member::RestrictChatMemberRequest {
        api.restrict_chat_member(self.id, user_id, permissions)
    }
}
