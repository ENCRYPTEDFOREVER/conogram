use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api, entities::menu_button::MenuButton, errors::ConogramError, impl_into_future,
    request::RequestT,
};

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
    api: &'a Api,
    params: SetChatMenuButtonParams,
}

impl RequestT for SetChatMenuButtonRequest<'_> {
    type ParamsType = SetChatMenuButtonParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setChatMenuButton"
    }
    fn get_api_ref(&self) -> &Api {
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
    pub fn new(api: &'a Api) -> Self {
        Self {
            api,
            params: SetChatMenuButtonParams {
                chat_id: Option::default(),
                menu_button: Option::default(),
            },
        }
    }

    ///Unique identifier for the target private chat. If not specified, default bot's menu button will be changed
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<i64>) -> Self {
        self.params.chat_id = Some(chat_id.into());
        self
    }

    ///A JSON-serialized object for the bot's new menu button. Defaults to [MenuButtonDefault](https://core.telegram.org/bots/api/#menubuttondefault)
    #[must_use]
    pub fn menu_button(mut self, menu_button: impl Into<MenuButton>) -> Self {
        self.params.menu_button = Some(menu_button.into());
        self
    }
}

impl Api {
    ///Use this method to change the bot's menu button in a private chat, or the default menu button. Returns *True* on success.
    pub fn set_chat_menu_button(&self) -> SetChatMenuButtonRequest {
        SetChatMenuButtonRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
