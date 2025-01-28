use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api, entities::misc::chat_id::ChatId, errors::ConogramError, impl_into_future,
    request::RequestT, utils::deserialize_utils::is_false,
};

#[derive(Debug, Clone, Serialize)]
pub struct PromoteChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: i64,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_anonymous: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_manage_chat: bool,
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
    pub can_post_stories: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_edit_stories: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_delete_stories: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_post_messages: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_edit_messages: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_pin_messages: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_manage_topics: bool,
}

impl_into_future!(PromoteChatMemberRequest<'a>);

///Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass *False* for all boolean parameters to demote a user. Returns *True* on success.
#[derive(Clone)]
pub struct PromoteChatMemberRequest<'a> {
    api: &'a Api,
    params: PromoteChatMemberParams,
}

impl RequestT for PromoteChatMemberRequest<'_> {
    type ParamsType = PromoteChatMemberParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "promoteChatMember"
    }
    fn get_api_ref(&self) -> &Api {
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
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: PromoteChatMemberParams {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
                is_anonymous: bool::default(),
                can_manage_chat: bool::default(),
                can_delete_messages: bool::default(),
                can_manage_video_chats: bool::default(),
                can_restrict_members: bool::default(),
                can_promote_members: bool::default(),
                can_change_info: bool::default(),
                can_invite_users: bool::default(),
                can_post_stories: bool::default(),
                can_edit_stories: bool::default(),
                can_delete_stories: bool::default(),
                can_post_messages: bool::default(),
                can_edit_messages: bool::default(),
                can_pin_messages: bool::default(),
                can_manage_topics: bool::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier of the target user
    #[must_use]
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///Pass *True* if the administrator's presence in the chat is hidden
    #[must_use]
    pub fn is_anonymous(mut self, is_anonymous: impl Into<bool>) -> Self {
        self.params.is_anonymous = is_anonymous.into();
        self
    }

    ///Pass *True* if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages and ignore slow mode. Implied by any other administrator privilege.
    #[must_use]
    pub fn can_manage_chat(mut self, can_manage_chat: impl Into<bool>) -> Self {
        self.params.can_manage_chat = can_manage_chat.into();
        self
    }

    ///Pass *True* if the administrator can delete messages of other users
    #[must_use]
    pub fn can_delete_messages(mut self, can_delete_messages: impl Into<bool>) -> Self {
        self.params.can_delete_messages = can_delete_messages.into();
        self
    }

    ///Pass *True* if the administrator can manage video chats
    #[must_use]
    pub fn can_manage_video_chats(mut self, can_manage_video_chats: impl Into<bool>) -> Self {
        self.params.can_manage_video_chats = can_manage_video_chats.into();
        self
    }

    ///Pass *True* if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    #[must_use]
    pub fn can_restrict_members(mut self, can_restrict_members: impl Into<bool>) -> Self {
        self.params.can_restrict_members = can_restrict_members.into();
        self
    }

    ///Pass *True* if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[must_use]
    pub fn can_promote_members(mut self, can_promote_members: impl Into<bool>) -> Self {
        self.params.can_promote_members = can_promote_members.into();
        self
    }

    ///Pass *True* if the administrator can change chat title, photo and other settings
    #[must_use]
    pub fn can_change_info(mut self, can_change_info: impl Into<bool>) -> Self {
        self.params.can_change_info = can_change_info.into();
        self
    }

    ///Pass *True* if the administrator can invite new users to the chat
    #[must_use]
    pub fn can_invite_users(mut self, can_invite_users: impl Into<bool>) -> Self {
        self.params.can_invite_users = can_invite_users.into();
        self
    }

    ///Pass *True* if the administrator can post stories to the chat
    #[must_use]
    pub fn can_post_stories(mut self, can_post_stories: impl Into<bool>) -> Self {
        self.params.can_post_stories = can_post_stories.into();
        self
    }

    ///Pass *True* if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    #[must_use]
    pub fn can_edit_stories(mut self, can_edit_stories: impl Into<bool>) -> Self {
        self.params.can_edit_stories = can_edit_stories.into();
        self
    }

    ///Pass *True* if the administrator can delete stories posted by other users
    #[must_use]
    pub fn can_delete_stories(mut self, can_delete_stories: impl Into<bool>) -> Self {
        self.params.can_delete_stories = can_delete_stories.into();
        self
    }

    ///Pass *True* if the administrator can post messages in the channel, or access channel statistics; for channels only
    #[must_use]
    pub fn can_post_messages(mut self, can_post_messages: impl Into<bool>) -> Self {
        self.params.can_post_messages = can_post_messages.into();
        self
    }

    ///Pass *True* if the administrator can edit messages of other users and can pin messages; for channels only
    #[must_use]
    pub fn can_edit_messages(mut self, can_edit_messages: impl Into<bool>) -> Self {
        self.params.can_edit_messages = can_edit_messages.into();
        self
    }

    ///Pass *True* if the administrator can pin messages; for supergroups only
    #[must_use]
    pub fn can_pin_messages(mut self, can_pin_messages: impl Into<bool>) -> Self {
        self.params.can_pin_messages = can_pin_messages.into();
        self
    }

    ///Pass *True* if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    #[must_use]
    pub fn can_manage_topics(mut self, can_manage_topics: impl Into<bool>) -> Self {
        self.params.can_manage_topics = can_manage_topics.into();
        self
    }
}

impl Api {
    ///Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass *False* for all boolean parameters to demote a user. Returns *True* on success.
    pub fn promote_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<i64>,
    ) -> PromoteChatMemberRequest {
        PromoteChatMemberRequest::new(self, chat_id, user_id)
    }
}

// Divider: all content below this line will be preserved after code regen
