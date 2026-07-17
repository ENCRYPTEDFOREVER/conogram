use serde::{Deserialize, Serialize};

use crate::utils::deserialize_utils::is_false;

/// This object represents a chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chat)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chat {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,

    /// Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub type_: ChatType,

    /// *Optional*. Title, for supergroups, channels and group chats
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// *Optional*. Username, for private chats, supergroups and channels if available
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// *Optional*. First name of the other party in a private chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// *Optional*. Last name of the other party in a private chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// *Optional*. *True*, if the supergroup chat is a forum (has [topics](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_forum: bool,

    /// *Optional*. *True*, if the chat is the direct messages chat of a channel
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_direct_messages: bool,
}

/// Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatType {
    /// `private`
    #[default]
    #[serde(rename = "private")]
    Private,

    /// `group`
    #[serde(rename = "group")]
    Group,

    /// `supergroup`
    #[serde(rename = "supergroup")]
    Supergroup,

    /// `channel`
    #[serde(rename = "channel")]
    Channel,
}

// Divider: all content below this line will be preserved after code regen
use super::{chat_full_info::ChatFullInfo, misc::input_file::InputFile};
use crate::{
    api::Api,
    entities::{chat_permissions::ChatPermissions, misc::chat_id::CHAT_ID_CONST},
    impl_trait,
    methods::{
        approve_chat_join_request::ApproveChatJoinRequestRequest,
        ban_chat_member::BanChatMemberRequest, ban_chat_sender_chat::BanChatSenderChatRequest,
        create_chat_invite_link::CreateChatInviteLinkRequest,
        decline_chat_join_request::DeclineChatJoinRequestRequest,
        delete_all_message_reactions::DeleteAllMessageReactionsRequest,
        delete_chat_photo::DeleteChatPhotoRequest,
        delete_chat_sticker_set::DeleteChatStickerSetRequest,
        edit_chat_invite_link::EditChatInviteLinkRequest,
        export_chat_invite_link::ExportChatInviteLinkRequest, get_chat::GetChatRequest,
        get_chat_administrators::GetChatAdministratorsRequest,
        get_chat_member::GetChatMemberRequest, get_chat_member_count::GetChatMemberCountRequest,
        leave_chat::LeaveChatRequest, promote_chat_member::PromoteChatMemberRequest,
        restrict_chat_member::RestrictChatMemberRequest,
        revoke_chat_invite_link::RevokeChatInviteLinkRequest,
        send_chat_action::SendChatActionRequest,
        set_chat_administrator_custom_title::SetChatAdministratorCustomTitleRequest,
        set_chat_description::SetChatDescriptionRequest,
        set_chat_permissions::SetChatPermissionsRequest, set_chat_photo::SetChatPhotoRequest,
        set_chat_sticker_set::SetChatStickerSetRequest, set_chat_title::SetChatTitleRequest,
        unban_chat_member::UnbanChatMemberRequest,
        unban_chat_sender_chat::UnbanChatSenderChatRequest,
        unpin_all_chat_messages::UnpinAllChatMessagesRequest,
    },
};

impl Chat {
    // Returns Chat's title for groups and User::full_name for private chats
    #[must_use]
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
    fn full_name(&self) -> impl AsRef<str>;
    fn username(&self) -> Option<impl AsRef<str>>;

    fn get_url(&self) -> String {
        if let Some(username) = self.username() {
            format!("https://t.me/{}", username.as_ref())
        } else {
            // message_id 999999999 is used to allow the link to work in all clients
            format!("https://t.me/c/{}/999999999", -self.id() - 1000000000000)
        }
    }

    /// Use this method to clear the list of pinned messages in a chat. In private chats and channel direct messages chats, no additional rights are required to unpin all pinned messages. Conversely, the bot must be an administrator with the 'can\_pin\_messages' right or the 'can\_edit\_messages' right to unpin all pinned messages in groups and channels respectively. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#unpinallchatmessages)
    fn unpin_all_messages<'a>(&'a self, api: &'a Api) -> UnpinAllChatMessagesRequest<'a> {
        api.unpin_all_chat_messages(self.id())
    }

    /// Use this method to get the number of members in a chat. Returns *Integer* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#getchatmembercount)
    fn get_member_count<'a>(&'a self, api: &'a Api) -> GetChatMemberCountRequest<'a> {
        api.get_chat_member_count(self.id())
    }

    /// Use this method to get a list of administrators in a chat. Returns an Array of [ChatMember](https://core.telegram.org/bots/api/#chatmember) objects.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#getchatadministrators)
    fn get_administrators<'a>(&'a self, api: &'a Api) -> GetChatAdministratorsRequest<'a> {
        api.get_chat_administrators(self.id())
    }

    /// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a [ChatMember](https://core.telegram.org/bots/api/#chatmember) object on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#getchatmember)
    fn get_member<'a>(&'a self, api: &'a Api, user_id: impl Into<i64>) -> GetChatMemberRequest<'a> {
        api.get_chat_member(self.id(), user_id)
    }

    /// Use this method to get up-to-date information about the chat. Returns a [ChatFullInfo](https://core.telegram.org/bots/api/#chatfullinfo) object on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#getchat)
    fn get_full<'a>(&'a self, api: &'a Api) -> GetChatRequest<'a> {
        api.get_chat(self.id())
    }

    /// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless [unbanned](https://core.telegram.org/bots/api/#unbanchatmember) first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#banchatmember)
    fn ban_member<'a>(&'a self, api: &'a Api, user_id: impl Into<i64>) -> BanChatMemberRequest<'a> {
        api.ban_chat_member(self.id(), user_id)
    }

    /// Use this method to unban a previously banned user in a supergroup or channel. The user will **not** return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be **removed** from the chat. If you don't want this, use the parameter *only\_if\_banned*. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#unbanchatmember)
    fn unban_member<'a>(
        &'a self,
        api: &'a Api,
        user_id: impl Into<i64>,
    ) -> UnbanChatMemberRequest<'a> {
        api.unban_chat_member(self.id(), user_id)
    }

    /// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is [unbanned](https://core.telegram.org/bots/api/#unbanchatsenderchat), the owner of the banned chat won't be able to send messages on behalf of **any of their channels**. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#banchatsenderchat)
    fn ban_sender_chat<'a>(
        &'a self,
        api: &'a Api,
        sender_chat_id: impl Into<i64>,
    ) -> BanChatSenderChatRequest<'a> {
        api.ban_chat_sender_chat(self.id(), sender_chat_id)
    }

    /// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#unbanchatsenderchat)
    fn unban_sender_chat<'a>(
        &'a self,
        api: &'a Api,
        sender_chat_id: impl Into<i64>,
    ) -> UnbanChatSenderChatRequest<'a> {
        api.unban_chat_sender_chat(self.id(), sender_chat_id)
    }

    /// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#setchatadministratorcustomtitle)
    fn set_administrator_custom_title<'a>(
        &'a self,
        api: &'a Api,
        user_id: impl Into<i64>,
        custom_title: impl Into<String>,
    ) -> SetChatAdministratorCustomTitleRequest<'a> {
        api.set_chat_administrator_custom_title(self.id(), user_id, custom_title)
    }

    /// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#setchattitle)
    fn set_title<'a>(&'a self, api: &'a Api, title: impl Into<String>) -> SetChatTitleRequest<'a> {
        api.set_chat_title(self.id(), title)
    }

    /// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#setchatdescription)
    fn set_description<'a>(
        &'a self,
        api: &'a Api,
        description: Option<impl Into<String>>,
    ) -> SetChatDescriptionRequest<'a> {
        if let Some(description) = description {
            api.set_chat_description(self.id()).description(description)
        } else {
            api.set_chat_description(self.id())
        }
    }

    /// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the *can\_restrict\_members* administrator rights. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#setchatpermissions)
    fn set_permissions<'a>(
        &'a self,
        api: &'a Api,
        permissions: impl Into<ChatPermissions>,
    ) -> SetChatPermissionsRequest<'a> {
        api.set_chat_permissions(self.id(), permissions)
    }

    /// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#setchatphoto)
    fn set_photo<'a>(
        &'a self,
        api: &'a Api,
        photo: impl Into<InputFile>,
    ) -> SetChatPhotoRequest<'a> {
        api.set_chat_photo(self.id(), photo)
    }

    /// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\_set\_sticker\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#setchatstickerset)
    fn set_sticker_set<'a>(
        &'a self,
        api: &'a Api,
        sticker_set_name: impl Into<String>,
    ) -> SetChatStickerSetRequest<'a> {
        api.set_chat_sticker_set(self.id(), sticker_set_name)
    }

    /// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns *True* on success.
    ///
    /// Example: The [ImageBot](https://t.me/imagebot) needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use [sendChatAction](https://core.telegram.org/bots/api/#sendchataction) with *action* = *upload\_photo*. The user will see a “sending photo” status for the bot.
    ///
    /// We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#sendchataction)
    fn send_action<'a>(
        &'a self,
        api: &'a Api,
        action: impl Into<crate::methods::send_chat_action::ChatAction>,
    ) -> SendChatActionRequest<'a> {
        api.send_chat_action(self.id(), action)
    }

    /// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#editchatinvitelink)
    fn edit_invite_link<'a>(
        &'a self,
        api: &'a Api,
        invite_link: impl Into<String>,
    ) -> EditChatInviteLinkRequest<'a> {
        api.edit_chat_invite_link(self.id(), invite_link)
    }

    /// Use this method for your bot to leave a group, supergroup or channel. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#leavechat)
    fn leave<'a>(&'a self, api: &'a Api) -> LeaveChatRequest<'a> {
        api.leave_chat(self.id())
    }

    /// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#deletechatphoto)
    fn delete_photo<'a>(&'a self, api: &'a Api) -> DeleteChatPhotoRequest<'a> {
        api.delete_chat_photo(self.id())
    }

    /// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\_set\_sticker\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#deletechatstickerset)
    fn delete_sticker_set<'a>(&'a self, api: &'a Api) -> DeleteChatStickerSetRequest<'a> {
        api.delete_chat_sticker_set(self.id())
    }

    /// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as *String* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#exportchatinvitelink)
    fn export_invite_link<'a>(&'a self, api: &'a Api) -> ExportChatInviteLinkRequest<'a> {
        api.export_chat_invite_link(self.id())
    }

    /// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#revokechatinvitelink)
    fn revoke_invite_link<'a>(
        &'a self,
        api: &'a Api,
        invite_link: impl Into<String>,
    ) -> RevokeChatInviteLinkRequest<'a> {
        api.revoke_chat_invite_link(self.id(), invite_link)
    }

    /// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\_invite\_users* administrator right. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#approvechatjoinrequest)
    fn approve_join_request<'a>(
        &'a self,
        api: &'a Api,
        user_id: impl Into<i64>,
    ) -> ApproveChatJoinRequestRequest<'a> {
        api.approve_chat_join_request(self.id(), user_id)
    }

    /// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\_invite\_users* administrator right. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#declinechatjoinrequest)
    fn decline_join_request<'a>(
        &'a self,
        api: &'a Api,
        user_id: impl Into<i64>,
    ) -> DeclineChatJoinRequestRequest<'a> {
        api.decline_chat_join_request(self.id(), user_id)
    }

    /// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#createchatinvitelink)
    fn create_invite_link<'a>(&'a self, api: &'a Api) -> CreateChatInviteLinkRequest<'a> {
        api.create_chat_invite_link(self.id())
    }

    /// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass *False* for all boolean parameters to demote a user. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#promotechatmember)
    fn promote_member<'a>(
        &'a self,
        api: &'a Api,
        user_id: impl Into<i64>,
    ) -> PromoteChatMemberRequest<'a> {
        api.promote_chat_member(self.id(), user_id)
    }

    /// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass *True* for all permissions to lift restrictions from a user. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#restrictchatmember)
    fn restrict_member<'a>(
        &'a self,
        api: &'a Api,
        user_id: impl Into<i64>,
        permissions: impl Into<ChatPermissions>,
    ) -> RestrictChatMemberRequest<'a> {
        api.restrict_chat_member(self.id(), user_id, permissions)
    }

    /// Use this method to remove up to 10000 recent reactions in a group or a supergroup chat added by a given user or chat. The bot must have the 'can\_delete\_messages' administrator right in the chat. Returns *True* on success.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#deleteallmessagereactions)
    fn delete_all_message_reactions<'a>(
        &'a self,
        api: &'a Api,
        actor_id: impl Into<i64>,
    ) -> DeleteAllMessageReactionsRequest<'a> {
        let actor_id = actor_id.into();
        if actor_id < CHAT_ID_CONST {
            api.delete_all_message_reactions(self.id())
                .user_id(actor_id)
        } else {
            api.delete_all_message_reactions(self.id())
                .actor_chat_id(actor_id)
        }
    }
}

impl_trait!(TgChat for Chat {
    fn id(&self) -> i64 {
        self.id
    }

    fn full_name(&self) -> impl AsRef<str> {
        if let Some(title) = &self.title {
            title.to_owned()
        } else if let Some(first) = &self.first_name {
            if let Some(last) = &self.last_name {
                format!("{first} {last}")
            } else {
                first.to_owned()
            }
        } else {
            "No Title".to_owned()
        }
    }

    fn username(&self) -> Option<impl AsRef<str>> {
        self.username.as_ref()
    }
});

impl_trait!(TgChat for ChatFullInfo {
    fn id(&self) -> i64 {
        self.id
    }

    fn full_name(&self) -> impl AsRef<str> {
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

    fn username(&self) -> Option<impl AsRef<str>> {
        self.username.as_ref()
    }
});
