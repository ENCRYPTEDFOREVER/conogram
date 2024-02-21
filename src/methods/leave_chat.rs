use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct LeaveChatParams {
    pub chat_id: ChatId,
}

impl_into_future!(LeaveChatRequest<'a>);

///Use this method for your bot to leave a group, supergroup or channel. Returns *True* on success.
#[derive(Clone)]
pub struct LeaveChatRequest<'a> {
    api: &'a API,
    params: LeaveChatParams,
}

impl<'a> RequestT for LeaveChatRequest<'a> {
    type ParamsType = LeaveChatParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "leaveChat"
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
impl<'a> LeaveChatRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: LeaveChatParams {
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
    ///Use this method for your bot to leave a group, supergroup or channel. Returns *True* on success.
    pub fn leave_chat(&'a self, chat_id: impl Into<ChatId>) -> LeaveChatRequest {
        LeaveChatRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
