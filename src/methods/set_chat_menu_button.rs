use crate::api::API;
use crate::entities::menu_button::MenuButton;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetChatMenuButtonParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<MenuButton>,
}

impl_into_future!(SetChatMenuButtonRequest<'a>);

///Use this method to change the bot's menu button in a private chat, or the default menu button. Returns *True* on success.
#[derive(Clone)]
pub struct SetChatMenuButtonRequest<'a> {
    api: &'a API,
    params: SetChatMenuButtonParams,
}

impl<'a> RequestT for SetChatMenuButtonRequest<'a> {
    type ParamsType = SetChatMenuButtonParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setChatMenuButton"
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
impl<'a> SetChatMenuButtonRequest<'a> {
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: SetChatMenuButtonParams {
                chat_id: Option::default(),
                menu_button: Option::default(),
            },
        }
    }

    ///Unique identifier for the target private chat. If not specified, default bot's menu button will be changed
    pub fn chat_id(mut self, chat_id: impl Into<i64>) -> Self {
        self.params.chat_id = Some(chat_id.into());
        self
    }

    ///A JSON-serialized object for the bot's new menu button. Defaults to [MenuButtonDefault](https://core.telegram.org/bots/api/#menubuttondefault)
    pub fn menu_button(mut self, menu_button: impl Into<MenuButton>) -> Self {
        self.params.menu_button = Some(menu_button.into());
        self
    }
}

impl<'a> API {
    ///Use this method to change the bot's menu button in a private chat, or the default menu button. Returns *True* on success.
    pub fn set_chat_menu_button(&'a self) -> SetChatMenuButtonRequest {
        SetChatMenuButtonRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
