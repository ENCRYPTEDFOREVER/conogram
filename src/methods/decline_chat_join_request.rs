use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct DeclineChatJoinRequestParams {
    pub chat_id: ChatId,
    pub user_id: i64,
}

impl_into_future!(DeclineChatJoinRequestRequest<'a>);

///Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\_invite\_users* administrator right. Returns *True* on success.
#[derive(Clone)]
pub struct DeclineChatJoinRequestRequest<'a> {
    api: &'a API,
    params: DeclineChatJoinRequestParams,
}

impl<'a> RequestT for DeclineChatJoinRequestRequest<'a> {
    type ParamsType = DeclineChatJoinRequestParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "declineChatJoinRequest"
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
impl<'a> DeclineChatJoinRequestRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId, user_id: i64) -> Self {
        Self {
            api,
            params: DeclineChatJoinRequestParams { chat_id, user_id },
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
}

impl<'a> API {
    ///Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\_invite\_users* administrator right. Returns *True* on success.
    pub fn decline_chat_join_request(
        &'a self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<i64>,
    ) -> DeclineChatJoinRequestRequest {
        DeclineChatJoinRequestRequest::new(self, chat_id.into(), user_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
