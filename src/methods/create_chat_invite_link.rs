use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api,
    entities::{chat_invite_link::ChatInviteLink, misc::chat_id::ChatId},
    errors::ConogramError,
    impl_into_future,
    request::RequestT,
    utils::deserialize_utils::is_false,
};

#[derive(Debug, Clone, Serialize)]
pub struct CreateChatInviteLinkParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub creates_join_request: bool,
}

impl_into_future!(CreateChatInviteLinkRequest<'a>);

///Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
#[derive(Clone)]
pub struct CreateChatInviteLinkRequest<'a> {
    api: &'a Api,
    params: CreateChatInviteLinkParams,
}

impl RequestT for CreateChatInviteLinkRequest<'_> {
    type ParamsType = CreateChatInviteLinkParams;
    type ReturnType = ChatInviteLink;
    fn get_name() -> &'static str {
        "createChatInviteLink"
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
impl<'a> CreateChatInviteLinkRequest<'a> {
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: CreateChatInviteLinkParams {
                chat_id: chat_id.into(),
                name: Option::default(),
                expire_date: Option::default(),
                member_limit: Option::default(),
                creates_join_request: bool::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Invite link name; 0-32 characters
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    ///Point in time (Unix timestamp) when the link will expire
    #[must_use]
    pub fn expire_date(mut self, expire_date: impl Into<i64>) -> Self {
        self.params.expire_date = Some(expire_date.into());
        self
    }

    ///The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[must_use]
    pub fn member_limit(mut self, member_limit: impl Into<i64>) -> Self {
        self.params.member_limit = Some(member_limit.into());
        self
    }

    ///*True*, if users joining the chat via the link need to be approved by chat administrators. If *True*, *member\_limit* can't be specified
    #[must_use]
    pub fn creates_join_request(mut self, creates_join_request: impl Into<bool>) -> Self {
        self.params.creates_join_request = creates_join_request.into();
        self
    }
}

impl Api {
    ///Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    pub fn create_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> CreateChatInviteLinkRequest {
        CreateChatInviteLinkRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
