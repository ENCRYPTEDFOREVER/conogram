use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct UnbanChatSenderChatParams {
    pub chat_id: ChatId,
    pub sender_chat_id: i64,
}

impl_into_future!(UnbanChatSenderChatRequest<'a>);

///Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns *True* on success.
#[derive(Clone)]
pub struct UnbanChatSenderChatRequest<'a> {
    api: &'a API,
    params: UnbanChatSenderChatParams,
}

impl<'a> RequestT for UnbanChatSenderChatRequest<'a> {
    type ParamsType = UnbanChatSenderChatParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "unbanChatSenderChat"
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
impl<'a> UnbanChatSenderChatRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, sender_chat_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: UnbanChatSenderChatParams {
                chat_id: chat_id.into(),
                sender_chat_id: sender_chat_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier of the target sender chat
    #[must_use]
    pub fn sender_chat_id(mut self, sender_chat_id: impl Into<i64>) -> Self {
        self.params.sender_chat_id = sender_chat_id.into();
        self
    }
}

impl API {
    ///Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns *True* on success.
    pub fn unban_chat_sender_chat(
        &self,
        chat_id: impl Into<ChatId>,
        sender_chat_id: impl Into<i64>,
    ) -> UnbanChatSenderChatRequest {
        UnbanChatSenderChatRequest::new(self, chat_id, sender_chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
