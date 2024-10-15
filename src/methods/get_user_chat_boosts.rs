use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API,
    entities::{misc::chat_id::ChatId, user_chat_boosts::UserChatBoosts},
    errors::ConogramError,
    impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetUserChatBoostsParams {
    pub chat_id: ChatId,
    pub user_id: i64,
}

impl_into_future!(GetUserChatBoostsRequest<'a>);

///Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a [UserChatBoosts](https://core.telegram.org/bots/api/#userchatboosts) object.
#[derive(Clone)]
pub struct GetUserChatBoostsRequest<'a> {
    api: &'a API,
    params: GetUserChatBoostsParams,
}

impl<'a> RequestT for GetUserChatBoostsRequest<'a> {
    type ParamsType = GetUserChatBoostsParams;
    type ReturnType = UserChatBoosts;
    fn get_name() -> &'static str {
        "getUserChatBoosts"
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
impl<'a> GetUserChatBoostsRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: GetUserChatBoostsParams {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
            },
        }
    }

    ///Unique identifier for the chat or username of the channel (in the format `@channelusername`)
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
    ///Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a [UserChatBoosts](https://core.telegram.org/bots/api/#userchatboosts) object.
    pub fn get_user_chat_boosts(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<i64>,
    ) -> GetUserChatBoostsRequest {
        GetUserChatBoostsRequest::new(self, chat_id, user_id)
    }
}

// Divider: all content below this line will be preserved after code regen
