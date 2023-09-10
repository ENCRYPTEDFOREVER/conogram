use crate::api::API;
use crate::entities::chat_member::ChatMember;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct GetChatAdministratorsParams {
    pub chat_id: ChatId,
}

impl_into_future!(GetChatAdministratorsRequest<'a>);

///Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of [ChatMember](https://core.telegram.org/bots/api/#chatmember) objects.
#[derive(Clone)]
pub struct GetChatAdministratorsRequest<'a> {
    api: &'a API,
    params: GetChatAdministratorsParams,
}

impl<'a> RequestT for GetChatAdministratorsRequest<'a> {
    type ParamsType = GetChatAdministratorsParams;
    type ReturnType = Vec<ChatMember>;
    fn get_name() -> &'static str {
        "getChatAdministrators"
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
impl<'a> GetChatAdministratorsRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId) -> Self {
        Self {
            api,
            params: GetChatAdministratorsParams { chat_id },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }
}

impl<'a> API {
    ///Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of [ChatMember](https://core.telegram.org/bots/api/#chatmember) objects.
    pub fn get_chat_administrators(
        &'a self,
        chat_id: impl Into<ChatId>,
    ) -> GetChatAdministratorsRequest {
        GetChatAdministratorsRequest::new(self, chat_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
