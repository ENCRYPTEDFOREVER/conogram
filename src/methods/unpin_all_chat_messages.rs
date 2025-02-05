


use serde::Serialize;

use crate::{
    api::Api, entities::misc::chat_id::ChatId,  impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]

pub struct UnpinAllChatMessagesParams {
    pub chat_id: ChatId,
}

impl_into_future!(UnpinAllChatMessagesRequest<'a>);

///Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\_pin\_messages' administrator right in a supergroup or 'can\_edit\_messages' administrator right in a channel. Returns *True* on success.
#[derive(Clone)]
pub struct UnpinAllChatMessagesRequest<'a> {
    api: &'a Api,
    params: UnpinAllChatMessagesParams,
}

impl RequestT for UnpinAllChatMessagesRequest<'_> {
    type ParamsType = UnpinAllChatMessagesParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "unpinAllChatMessages"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> UnpinAllChatMessagesRequest<'a> {
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: UnpinAllChatMessagesParams {
                chat_id: chat_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }
}

impl Api {
    ///Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\_pin\_messages' administrator right in a supergroup or 'can\_edit\_messages' administrator right in a channel. Returns *True* on success.
    pub fn unpin_all_chat_messages(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> UnpinAllChatMessagesRequest {
        UnpinAllChatMessagesRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
