use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetChatDescriptionParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl_into_future!(SetChatDescriptionRequest<'a>);

///Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
#[derive(Clone)]
pub struct SetChatDescriptionRequest<'a> {
    api: &'a API,
    params: SetChatDescriptionParams,
}

impl<'a> RequestT for SetChatDescriptionRequest<'a> {
    type ParamsType = SetChatDescriptionParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setChatDescription"
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
impl<'a> SetChatDescriptionRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: SetChatDescriptionParams {
                chat_id: chat_id.into(),
                description: Option::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///New chat description, 0-255 characters
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.params.description = Some(description.into());
        self
    }
}

impl<'a> API {
    ///Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    pub fn set_chat_description(&'a self, chat_id: impl Into<ChatId>) -> SetChatDescriptionRequest {
        SetChatDescriptionRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
