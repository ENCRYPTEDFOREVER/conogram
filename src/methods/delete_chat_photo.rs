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
pub struct DeleteChatPhotoParams {
    pub chat_id: ChatId,
}

impl_into_future!(DeleteChatPhotoRequest<'a>);

///Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
#[derive(Clone)]
pub struct DeleteChatPhotoRequest<'a> {
    api: &'a API,
    params: DeleteChatPhotoParams,
}

impl<'a> RequestT for DeleteChatPhotoRequest<'a> {
    type ParamsType = DeleteChatPhotoParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "deleteChatPhoto"
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
impl<'a> DeleteChatPhotoRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: DeleteChatPhotoParams {
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

impl API {
    ///Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    pub fn delete_chat_photo(&self, chat_id: impl Into<ChatId>) -> DeleteChatPhotoRequest {
        DeleteChatPhotoRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
