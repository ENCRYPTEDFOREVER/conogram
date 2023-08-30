use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct PromoteChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: i64,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_anonymous: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_manage_chat: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_post_messages: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_edit_messages: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_delete_messages: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_manage_video_chats: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_restrict_members: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_promote_members: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_change_info: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_invite_users: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_pin_messages: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_manage_topics: bool,
}

impl_into_future!(PromoteChatMemberRequest<'a>);

///Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass *False* for all boolean parameters to demote a user. Returns *True* on success.
#[derive(Clone)]
pub struct PromoteChatMemberRequest<'a> {
    api: &'a API,
    params: PromoteChatMemberParams,
}

impl<'a> RequestT for PromoteChatMemberRequest<'a> {
    type ParamsType = PromoteChatMemberParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "promoteChatMember"
    }
    fn get_api_ref(&self) -> &API {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        false
    }
}
impl<'a> PromoteChatMemberRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId, user_id: i64) -> Self {
        Self {
            api,
            params: PromoteChatMemberParams {
                chat_id,
                user_id,
                is_anonymous: bool::default(),
                can_manage_chat: bool::default(),
                can_post_messages: bool::default(),
                can_edit_messages: bool::default(),
                can_delete_messages: bool::default(),
                can_manage_video_chats: bool::default(),
                can_restrict_members: bool::default(),
                can_promote_members: bool::default(),
                can_change_info: bool::default(),
                can_invite_users: bool::default(),
                can_pin_messages: bool::default(),
                can_manage_topics: bool::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier of the target user
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///Pass *True* if the administrator's presence in the chat is hidden
    pub fn is_anonymous(mut self, is_anonymous: impl Into<bool>) -> Self {
        self.params.is_anonymous = is_anonymous.into();
        self
    }

    ///Pass *True* if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    pub fn can_manage_chat(mut self, can_manage_chat: impl Into<bool>) -> Self {
        self.params.can_manage_chat = can_manage_chat.into();
        self
    }

    ///Pass *True* if the administrator can create channel posts, channels only
    pub fn can_post_messages(mut self, can_post_messages: impl Into<bool>) -> Self {
        self.params.can_post_messages = can_post_messages.into();
        self
    }

    ///Pass *True* if the administrator can edit messages of other users and can pin messages, channels only
    pub fn can_edit_messages(mut self, can_edit_messages: impl Into<bool>) -> Self {
        self.params.can_edit_messages = can_edit_messages.into();
        self
    }

    ///Pass *True* if the administrator can delete messages of other users
    pub fn can_delete_messages(mut self, can_delete_messages: impl Into<bool>) -> Self {
        self.params.can_delete_messages = can_delete_messages.into();
        self
    }

    ///Pass *True* if the administrator can manage video chats
    pub fn can_manage_video_chats(mut self, can_manage_video_chats: impl Into<bool>) -> Self {
        self.params.can_manage_video_chats = can_manage_video_chats.into();
        self
    }

    ///Pass *True* if the administrator can restrict, ban or unban chat members
    pub fn can_restrict_members(mut self, can_restrict_members: impl Into<bool>) -> Self {
        self.params.can_restrict_members = can_restrict_members.into();
        self
    }

    ///Pass *True* if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him)
    pub fn can_promote_members(mut self, can_promote_members: impl Into<bool>) -> Self {
        self.params.can_promote_members = can_promote_members.into();
        self
    }

    ///Pass *True* if the administrator can change chat title, photo and other settings
    pub fn can_change_info(mut self, can_change_info: impl Into<bool>) -> Self {
        self.params.can_change_info = can_change_info.into();
        self
    }

    ///Pass *True* if the administrator can invite new users to the chat
    pub fn can_invite_users(mut self, can_invite_users: impl Into<bool>) -> Self {
        self.params.can_invite_users = can_invite_users.into();
        self
    }

    ///Pass *True* if the administrator can pin messages, supergroups only
    pub fn can_pin_messages(mut self, can_pin_messages: impl Into<bool>) -> Self {
        self.params.can_pin_messages = can_pin_messages.into();
        self
    }

    ///Pass *True* if the user is allowed to create, rename, close, and reopen forum topics, supergroups only
    pub fn can_manage_topics(mut self, can_manage_topics: impl Into<bool>) -> Self {
        self.params.can_manage_topics = can_manage_topics.into();
        self
    }
}

impl<'a> API {
    ///Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass *False* for all boolean parameters to demote a user. Returns *True* on success.
    pub fn promote_chat_member(
        &'a self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<i64>,
    ) -> PromoteChatMemberRequest {
        PromoteChatMemberRequest::new(self, chat_id.into(), user_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
