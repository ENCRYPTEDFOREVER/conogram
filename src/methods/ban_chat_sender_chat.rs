use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct BanChatSenderChatParams {
    pub chat_id: ChatId,
    pub sender_chat_id: i64,
}

impl_into_future!(BanChatSenderChatRequest<'a>);

///Use this method to ban a channel chat in a supergroup or a channel. Until the chat is [unbanned](https://core.telegram.org/bots/api/#unbanchatsenderchat), the owner of the banned chat won't be able to send messages on behalf of **any of their channels**. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns *True* on success.
#[derive(Clone)]
pub struct BanChatSenderChatRequest<'a> {
    api: &'a API,
    params: BanChatSenderChatParams,
}

impl<'a> RequestT for BanChatSenderChatRequest<'a> {
    type ParamsType = BanChatSenderChatParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "banChatSenderChat"
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
impl<'a> BanChatSenderChatRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId, sender_chat_id: i64) -> Self {
        Self {
            api,
            params: BanChatSenderChatParams {
                chat_id,
                sender_chat_id,
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier of the target sender chat
    pub fn sender_chat_id(mut self, sender_chat_id: impl Into<i64>) -> Self {
        self.params.sender_chat_id = sender_chat_id.into();
        self
    }
}

impl<'a> API {
    ///Use this method to ban a channel chat in a supergroup or a channel. Until the chat is [unbanned](https://core.telegram.org/bots/api/#unbanchatsenderchat), the owner of the banned chat won't be able to send messages on behalf of **any of their channels**. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns *True* on success.
    pub fn ban_chat_sender_chat(
        &'a self,
        chat_id: impl Into<ChatId>,
        sender_chat_id: impl Into<i64>,
    ) -> BanChatSenderChatRequest {
        BanChatSenderChatRequest::new(self, chat_id.into(), sender_chat_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
