use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct UnpinChatMessageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
}

impl_into_future!(UnpinChatMessageRequest<'a>);

///Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\_pin\_messages' administrator right in a supergroup or 'can\_edit\_messages' administrator right in a channel. Returns *True* on success.
#[derive(Clone)]
pub struct UnpinChatMessageRequest<'a> {
    api: &'a API,
    params: UnpinChatMessageParams,
}

impl<'a> RequestT for UnpinChatMessageRequest<'a> {
    type ParamsType = UnpinChatMessageParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "unpinChatMessage"
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
impl<'a> UnpinChatMessageRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: UnpinChatMessageParams {
                chat_id: chat_id.into(),
                business_connection_id: Option::default(),
                message_id: Option::default(),
            },
        }
    }

    ///Unique identifier of the business connection on behalf of which the message will be unpinned
    #[must_use]
    pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(business_connection_id.into());
        self
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Identifier of the message to unpin. Required if *business\_connection\_id* is specified. If not specified, the most recent pinned message (by sending date) will be unpinned.
    #[must_use]
    pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
        self.params.message_id = Some(message_id.into());
        self
    }
}

impl API {
    ///Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\_pin\_messages' administrator right in a supergroup or 'can\_edit\_messages' administrator right in a channel. Returns *True* on success.
    pub fn unpin_chat_message(&self, chat_id: impl Into<ChatId>) -> UnpinChatMessageRequest {
        UnpinChatMessageRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
