use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API,
    entities::{chat_full_info::ChatFullInfo, misc::chat_id::ChatId},
    errors::ConogramError,
    impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetChatParams {
    pub chat_id: ChatId,
}

impl_into_future!(GetChatRequest<'a>);

///Use this method to get up-to-date information about the chat. Returns a [ChatFullInfo](https://core.telegram.org/bots/api/#chatfullinfo) object on success.
#[derive(Clone)]
pub struct GetChatRequest<'a> {
    api: &'a API,
    params: GetChatParams,
}

impl<'a> RequestT for GetChatRequest<'a> {
    type ParamsType = GetChatParams;
    type ReturnType = ChatFullInfo;
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
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }
}

impl API {
    ///Use this method to get up-to-date information about the chat. Returns a [ChatFullInfo](https://core.telegram.org/bots/api/#chatfullinfo) object on success.
    pub fn get_chat(&self, chat_id: impl Into<ChatId>) -> GetChatRequest {
        GetChatRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
