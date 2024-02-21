use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct UnpinAllChatMessagesParams {
    pub chat_id: ChatId,
}

impl_into_future!(UnpinAllChatMessagesRequest<'a>);

///Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\_pin\_messages' administrator right in a supergroup or 'can\_edit\_messages' administrator right in a channel. Returns *True* on success.
#[derive(Clone)]
pub struct UnpinAllChatMessagesRequest<'a> {
    api: &'a API,
    params: UnpinAllChatMessagesParams,
}

impl<'a> RequestT for UnpinAllChatMessagesRequest<'a> {
    type ParamsType = UnpinAllChatMessagesParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "unpinAllChatMessages"
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
impl<'a> UnpinAllChatMessagesRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: UnpinAllChatMessagesParams {
                chat_id: chat_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }
}

impl<'a> API {
    ///Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\_pin\_messages' administrator right in a supergroup or 'can\_edit\_messages' administrator right in a channel. Returns *True* on success.
    pub fn unpin_all_chat_messages(
        &'a self,
        chat_id: impl Into<ChatId>,
    ) -> UnpinAllChatMessagesRequest {
        UnpinAllChatMessagesRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
