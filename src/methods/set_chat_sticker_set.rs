use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetChatStickerSetParams {
    pub chat_id: ChatId,
    pub sticker_set_name: String,
}

impl_into_future!(SetChatStickerSetRequest<'a>);

///Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\_set\_sticker\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
#[derive(Clone)]
pub struct SetChatStickerSetRequest<'a> {
    api: &'a API,
    params: SetChatStickerSetParams,
}

impl<'a> RequestT for SetChatStickerSetRequest<'a> {
    type ParamsType = SetChatStickerSetParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setChatStickerSet"
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
impl<'a> SetChatStickerSetRequest<'a> {
    pub fn new(
        api: &'a API,
        chat_id: impl Into<ChatId>,
        sticker_set_name: impl Into<String>,
    ) -> Self {
        Self {
            api,
            params: SetChatStickerSetParams {
                chat_id: chat_id.into(),
                sticker_set_name: sticker_set_name.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Name of the sticker set to be set as the group sticker set
    pub fn sticker_set_name(mut self, sticker_set_name: impl Into<String>) -> Self {
        self.params.sticker_set_name = sticker_set_name.into();
        self
    }
}

impl<'a> API {
    ///Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\_set\_sticker\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
    pub fn set_chat_sticker_set(
        &'a self,
        chat_id: impl Into<ChatId>,
        sticker_set_name: impl Into<String>,
    ) -> SetChatStickerSetRequest {
        SetChatStickerSetRequest::new(self, chat_id, sticker_set_name)
    }
}

// Divider: all content below this line will be preserved after code regen
