use crate::api::API;
use crate::entities::menu_button::MenuButton;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct GetChatMenuButtonParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
}

impl_into_future!(GetChatMenuButtonRequest<'a>);

///Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns [MenuButton](https://core.telegram.org/bots/api/#menubutton) on success.
#[derive(Clone)]
pub struct GetChatMenuButtonRequest<'a> {
    api: &'a API,
    params: GetChatMenuButtonParams,
}

impl<'a> RequestT for GetChatMenuButtonRequest<'a> {
    type ParamsType = GetChatMenuButtonParams;
    type ReturnType = MenuButton;
    fn get_name() -> &'static str {
        "getChatMenuButton"
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
impl<'a> GetChatMenuButtonRequest<'a> {
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: GetChatMenuButtonParams {
                chat_id: Option::default(),
            },
        }
    }

    ///Unique identifier for the target private chat. If not specified, default bot's menu button will be returned
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<i64>) -> Self {
        self.params.chat_id = Some(chat_id.into());
        self
    }
}

impl<'a> API {
    ///Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns [MenuButton](https://core.telegram.org/bots/api/#menubutton) on success.
    pub fn get_chat_menu_button(&'a self) -> GetChatMenuButtonRequest {
        GetChatMenuButtonRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
