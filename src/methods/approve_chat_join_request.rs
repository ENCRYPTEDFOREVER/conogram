use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API, entities::misc::chat_id::ChatId, errors::ConogramError, impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct ApproveChatJoinRequestParams {
    pub chat_id: ChatId,
    pub user_id: i64,
}

impl_into_future!(ApproveChatJoinRequestRequest<'a>);

///Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\_invite\_users* administrator right. Returns *True* on success.
#[derive(Clone)]
pub struct ApproveChatJoinRequestRequest<'a> {
    api: &'a API,
    params: ApproveChatJoinRequestParams,
}

impl<'a> RequestT for ApproveChatJoinRequestRequest<'a> {
    type ParamsType = ApproveChatJoinRequestParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "approveChatJoinRequest"
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
impl<'a> ApproveChatJoinRequestRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: ApproveChatJoinRequestParams {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
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
}

impl API {
    ///Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\_invite\_users* administrator right. Returns *True* on success.
    pub fn approve_chat_join_request(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<i64>,
    ) -> ApproveChatJoinRequestRequest {
        ApproveChatJoinRequestRequest::new(self, chat_id, user_id)
    }
}

// Divider: all content below this line will be preserved after code regen
