use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::entities::user_chat_boosts::UserChatBoosts;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

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
    ///Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a [UserChatBoosts](https://core.telegram.org/bots/api/#userchatboosts) object.
    pub fn get_user_chat_boosts(
        &'a self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<i64>,
    ) -> GetUserChatBoostsRequest {
        GetUserChatBoostsRequest::new(self, chat_id.into(), user_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
