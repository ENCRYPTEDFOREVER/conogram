use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api, entities::misc::chat_id::ChatId, errors::ConogramError, impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct LeaveChatParams {
    pub chat_id: ChatId,
}

impl_into_future!(LeaveChatRequest<'a>);

///Use this method for your bot to leave a group, supergroup or channel. Returns *True* on success.
#[derive(Clone)]
pub struct LeaveChatRequest<'a> {
    api: &'a Api,
    params: LeaveChatParams,
}

impl RequestT for LeaveChatRequest<'_> {
    type ParamsType = LeaveChatParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "leaveChat"
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
impl<'a> LeaveChatRequest<'a> {
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: LeaveChatParams {
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

impl Api {
    ///Use this method for your bot to leave a group, supergroup or channel. Returns *True* on success.
    pub fn leave_chat(&self, chat_id: impl Into<ChatId>) -> LeaveChatRequest {
        LeaveChatRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
