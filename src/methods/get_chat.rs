use crate::api::API;
use crate::entities::chat::Chat;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct GetChatParams {
    pub chat_id: ChatId,
}

impl_into_future!(GetChatRequest<'a>);

///Use this method to get up to date information about the chat. Returns a [Chat](https://core.telegram.org/bots/api/#chat) object on success.
#[derive(Clone)]
pub struct GetChatRequest<'a> {
    api: &'a API,
    params: GetChatParams,
}

impl<'a> RequestT for GetChatRequest<'a> {
    type ParamsType = GetChatParams;
    type ReturnType = Chat;
    fn get_name() -> &'static str {
        "getChat"
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
impl<'a> GetChatRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: GetChatParams {
                chat_id: chat_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }
}

impl<'a> API {
    ///Use this method to get up to date information about the chat. Returns a [Chat](https://core.telegram.org/bots/api/#chat) object on success.
    pub fn get_chat(&'a self, chat_id: impl Into<ChatId>) -> GetChatRequest {
        GetChatRequest::new(self, chat_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
