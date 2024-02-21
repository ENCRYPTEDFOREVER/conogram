use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetChatTitleParams {
    pub chat_id: ChatId,
    pub title: String,
}

impl_into_future!(SetChatTitleRequest<'a>);

///Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
#[derive(Clone)]
pub struct SetChatTitleRequest<'a> {
    api: &'a API,
    params: SetChatTitleParams,
}

impl<'a> RequestT for SetChatTitleRequest<'a> {
    type ParamsType = SetChatTitleParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setChatTitle"
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
impl<'a> SetChatTitleRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, title: impl Into<String>) -> Self {
        Self {
            api,
            params: SetChatTitleParams {
                chat_id: chat_id.into(),
                title: title.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///New chat title, 1-128 characters
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.params.title = title.into();
        self
    }
}

impl<'a> API {
    ///Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    pub fn set_chat_title(
        &'a self,
        chat_id: impl Into<ChatId>,
        title: impl Into<String>,
    ) -> SetChatTitleRequest {
        SetChatTitleRequest::new(self, chat_id, title)
    }
}

// Divider: all content below this line will be preserved after code regen
