use crate::api::API;
use crate::entities::chat_invite_link::ChatInviteLink;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct CreateChatInviteLinkParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub creates_join_request: bool,
}

impl_into_future!(CreateChatInviteLinkRequest<'a>);

///Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
#[derive(Clone)]
pub struct CreateChatInviteLinkRequest<'a> {
    api: &'a API,
    params: CreateChatInviteLinkParams,
}

impl<'a> RequestT for CreateChatInviteLinkRequest<'a> {
    type ParamsType = CreateChatInviteLinkParams;
    type ReturnType = ChatInviteLink;
    fn get_name() -> &'static str {
        "createChatInviteLink"
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
impl<'a> CreateChatInviteLinkRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId) -> Self {
        Self {
            api,
            params: CreateChatInviteLinkParams {
                chat_id,
                name: Option::default(),
                expire_date: Option::default(),
                member_limit: Option::default(),
                creates_join_request: bool::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Invite link name; 0-32 characters
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    ///Point in time (Unix timestamp) when the link will expire
    pub fn expire_date(mut self, expire_date: impl Into<i64>) -> Self {
        self.params.expire_date = Some(expire_date.into());
        self
    }

    ///The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    pub fn member_limit(mut self, member_limit: impl Into<i64>) -> Self {
        self.params.member_limit = Some(member_limit.into());
        self
    }

    ///*True*, if users joining the chat via the link need to be approved by chat administrators. If *True*, *member\_limit* can't be specified
    pub fn creates_join_request(mut self, creates_join_request: impl Into<bool>) -> Self {
        self.params.creates_join_request = creates_join_request.into();
        self
    }
}

impl<'a> API {
    ///Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    pub fn create_chat_invite_link(
        &'a self,
        chat_id: impl Into<ChatId>,
    ) -> CreateChatInviteLinkRequest {
        CreateChatInviteLinkRequest::new(self, chat_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
