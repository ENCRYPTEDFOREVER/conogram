use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///This object represents a chat.
///
///API Reference: [link](https://core.telegram.org/bots/api/#chat)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    ///Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,

    ///Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
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
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_forum: bool,
}

///Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
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
use super::chat_full_info::ChatFullInfo;
use super::misc::input_file::InputFile;
use crate::api::API;
use crate::entities::chat_permissions::ChatPermissions;
use crate::impl_trait;

use crate::methods::approve_chat_join_request::ApproveChatJoinRequestRequest;
use crate::methods::ban_chat_member::BanChatMemberRequest;
use crate::methods::ban_chat_sender_chat::BanChatSenderChatRequest;
use crate::methods::create_chat_invite_link::CreateChatInviteLinkRequest;
use crate::methods::decline_chat_join_request::DeclineChatJoinRequestRequest;
use crate::methods::delete_chat_photo::DeleteChatPhotoRequest;
use crate::methods::delete_chat_sticker_set::DeleteChatStickerSetRequest;
use crate::methods::edit_chat_invite_link::EditChatInviteLinkRequest;
use crate::methods::export_chat_invite_link::ExportChatInviteLinkRequest;
use crate::methods::get_chat::GetChatRequest;
use crate::methods::get_chat_administrators::GetChatAdministratorsRequest;
use crate::methods::get_chat_member::GetChatMemberRequest;
use crate::methods::get_chat_member_count::GetChatMemberCountRequest;
use crate::methods::leave_chat::LeaveChatRequest;
use crate::methods::promote_chat_member::PromoteChatMemberRequest;
use crate::methods::restrict_chat_member::RestrictChatMemberRequest;
use crate::methods::revoke_chat_invite_link::RevokeChatInviteLinkRequest;
use crate::methods::send_chat_action::SendChatActionRequest;
use crate::methods::set_chat_administrator_custom_title::SetChatAdministratorCustomTitleRequest;
use crate::methods::set_chat_description::SetChatDescriptionRequest;
use crate::methods::set_chat_permissions::SetChatPermissionsRequest;
use crate::methods::set_chat_photo::SetChatPhotoRequest;
use crate::methods::set_chat_sticker_set::SetChatStickerSetRequest;
use crate::methods::set_chat_title::SetChatTitleRequest;
use crate::methods::unban_chat_member::UnbanChatMemberRequest;
use crate::methods::unban_chat_sender_chat::UnbanChatSenderChatRequest;
use crate::methods::unpin_all_chat_messages::UnpinAllChatMessagesRequest;

impl Chat {
    // Returns Chat's title for groups and User::full_name for private chats
    pub fn full_name(&self) -> String {
        if let Some(title) = &self.title {
            title.clone()
        } else if let Some(first) = &self.first_name {
            if let Some(last) = &self.last_name {
                format!("{first} {last}")
            } else {
                first.clone()
            }
        } else {
            "No Title".to_owned()
        }
    }
}

pub trait TgChat {
    fn id(&self) -> i64;
    fn full_name(&self) -> String;
    fn username(&self) -> Option<&String>;

    fn get_url(&self) -> String {
        if let Some(username) = &self.username() {
            format!("https://t.me/{username}")
        } else {
            // message_id 999999999 is used to allow the link to work in all clients
            format!("https://t.me/c/{}/999999999", -self.id() - 1000000000000)
        }
    }

    fn unpin_all_messages<'a>(&'a self, api: &'a API) -> UnpinAllChatMessagesRequest<'a> {
        api.unpin_all_chat_messages(self.id())
    }

    fn get_member_count<'a>(&'a self, api: &'a API) -> GetChatMemberCountRequest<'a> {
        api.get_chat_member_count(self.id())
    }

    fn get_administrators<'a>(&'a self, api: &'a API) -> GetChatAdministratorsRequest<'a> {
        api.get_chat_administrators(self.id())
    }

    fn get_member<'a>(&'a self, api: &'a API, user_id: impl Into<i64>) -> GetChatMemberRequest<'a> {
        api.get_chat_member(self.id(), user_id)
    }

    fn get_full<'a>(&'a self, api: &'a API) -> GetChatRequest<'a> {
        api.get_chat(self.id())
    }

    fn ban_member<'a>(&'a self, api: &'a API, user_id: impl Into<i64>) -> BanChatMemberRequest<'a> {
        api.ban_chat_member(self.id(), user_id)
    }

    fn unban_member<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
    ) -> UnbanChatMemberRequest<'a> {
        api.unban_chat_member(self.id(), user_id)
    }

    fn ban_sender_chat<'a>(
        &'a self,
        api: &'a API,
        sender_chat_id: impl Into<i64>,
    ) -> BanChatSenderChatRequest<'a> {
        api.ban_chat_sender_chat(self.id(), sender_chat_id)
    }

    fn unban_sender_chat<'a>(
        &'a self,
        api: &'a API,
        sender_chat_id: impl Into<i64>,
    ) -> UnbanChatSenderChatRequest<'a> {
        api.unban_chat_sender_chat(self.id(), sender_chat_id)
    }

    fn set_administrator_custom_title<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
        custom_title: impl Into<String>,
    ) -> SetChatAdministratorCustomTitleRequest<'a> {
        api.set_chat_administrator_custom_title(self.id(), user_id, custom_title)
    }

    fn set_title<'a>(&'a self, api: &'a API, title: impl Into<String>) -> SetChatTitleRequest<'a> {
        api.set_chat_title(self.id(), title)
    }

    fn set_description<'a>(
        &'a self,
        api: &'a API,
        description: Option<impl Into<String>>,
    ) -> SetChatDescriptionRequest<'a> {
        if let Some(description) = description {
            api.set_chat_description(self.id()).description(description)
        } else {
            api.set_chat_description(self.id())
        }
    }

    fn set_permissions<'a>(
        &'a self,
        api: &'a API,
        permissions: impl Into<ChatPermissions>,
    ) -> SetChatPermissionsRequest<'a> {
        api.set_chat_permissions(self.id(), permissions)
    }

    fn set_photo<'a>(
        &'a self,
        api: &'a API,
        photo: impl Into<InputFile>,
    ) -> SetChatPhotoRequest<'a> {
        api.set_chat_photo(self.id(), photo)
    }

    fn set_sticker_set<'a>(
        &'a self,
        api: &'a API,
        sticker_set_name: impl Into<String>,
    ) -> SetChatStickerSetRequest<'a> {
        api.set_chat_sticker_set(self.id(), sticker_set_name)
    }

    fn send_action<'a>(
        &'a self,
        api: &'a API,
        action: impl Into<crate::methods::send_chat_action::SendChatActionAction>,
    ) -> SendChatActionRequest<'a> {
        api.send_chat_action(self.id(), action)
    }

    fn edit_invite_link<'a>(
        &'a self,
        api: &'a API,
        invite_link: impl Into<String>,
    ) -> EditChatInviteLinkRequest<'a> {
        api.edit_chat_invite_link(self.id(), invite_link)
    }

    fn leave<'a>(&'a self, api: &'a API) -> LeaveChatRequest<'a> {
        api.leave_chat(self.id())
    }

    fn delete_photo<'a>(&'a self, api: &'a API) -> DeleteChatPhotoRequest<'a> {
        api.delete_chat_photo(self.id())
    }

    fn delete_sticker_set<'a>(&'a self, api: &'a API) -> DeleteChatStickerSetRequest<'a> {
        api.delete_chat_sticker_set(self.id())
    }

    fn export_invite_link<'a>(&'a self, api: &'a API) -> ExportChatInviteLinkRequest<'a> {
        api.export_chat_invite_link(self.id())
    }

    fn revoke_invite_link<'a>(
        &'a self,
        api: &'a API,
        invite_link: impl Into<String>,
    ) -> RevokeChatInviteLinkRequest<'a> {
        api.revoke_chat_invite_link(self.id(), invite_link)
    }

    fn approve_join_request<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
    ) -> ApproveChatJoinRequestRequest<'a> {
        api.approve_chat_join_request(self.id(), user_id)
    }

    fn decline_join_request<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
    ) -> DeclineChatJoinRequestRequest<'a> {
        api.decline_chat_join_request(self.id(), user_id)
    }

    fn create_invite_link<'a>(&'a self, api: &'a API) -> CreateChatInviteLinkRequest<'a> {
        api.create_chat_invite_link(self.id())
    }

    fn promote_member<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
    ) -> PromoteChatMemberRequest<'a> {
        api.promote_chat_member(self.id(), user_id)
    }

    fn restrict_member<'a>(
        &'a self,
        api: &'a API,
        user_id: impl Into<i64>,
        permissions: impl Into<ChatPermissions>,
    ) -> RestrictChatMemberRequest<'a> {
        api.restrict_chat_member(self.id(), user_id, permissions)
    }
}

impl_trait!(TgChat for Chat {
    fn id(&self) -> i64 {
        self.id
    }

    fn full_name(&self) -> String {
        if let Some(title) = &self.title {
            title.clone()
        } else if let Some(first) = &self.first_name {
            if let Some(last) = &self.last_name {
                format!("{first} {last}")
            } else {
                first.clone()
            }
        } else {
            "No Title".to_owned()
        }
    }

    fn username(&self) -> Option<&String> {
        self.username.as_ref()
    }
});

impl_trait!(TgChat for ChatFullInfo {
    fn id(&self) -> i64 {
        self.id
    }

    fn full_name(&self) -> String {
        if let Some(title) = &self.title {
            title.clone()
        } else if let Some(first) = &self.first_name {
            if let Some(last) = &self.last_name {
                format!("{first} {last}")
            } else {
                first.clone()
            }
        } else {
            "No Title".to_owned()
        }
    }

    fn username(&self) -> Option<&String> {
        self.username.as_ref()
    }
});
