use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetChatAdministratorCustomTitleParams {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub custom_title: String,
}

impl_into_future!(SetChatAdministratorCustomTitleRequest<'a>);

///Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns *True* on success.
#[derive(Clone)]
pub struct SetChatAdministratorCustomTitleRequest<'a> {
    api: &'a API,
    params: SetChatAdministratorCustomTitleParams,
}

impl<'a> RequestT for SetChatAdministratorCustomTitleRequest<'a> {
    type ParamsType = SetChatAdministratorCustomTitleParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setChatAdministratorCustomTitle"
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
impl<'a> SetChatAdministratorCustomTitleRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId, user_id: i64, custom_title: String) -> Self {
        Self {
            api,
            params: SetChatAdministratorCustomTitleParams {
                chat_id,
                user_id,
                custom_title,
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier of the target user
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub fn custom_title(mut self, custom_title: impl Into<String>) -> Self {
        self.params.custom_title = custom_title.into();
        self
    }
}

impl<'a> API {
    ///Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns *True* on success.
    pub fn set_chat_administrator_custom_title(
        &'a self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<i64>,
        custom_title: impl Into<String>,
    ) -> SetChatAdministratorCustomTitleRequest {
        SetChatAdministratorCustomTitleRequest::new(
            self,
            chat_id.into(),
            user_id.into(),
            custom_title.into(),
        )
    }
}

// Divider: all content below this line will be preserved after code regen
