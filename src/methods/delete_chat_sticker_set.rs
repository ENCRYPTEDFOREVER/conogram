use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct DeleteChatStickerSetParams {
    pub chat_id: ChatId,
}

impl_into_future!(DeleteChatStickerSetRequest<'a>);

///Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\_set\_sticker\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
#[derive(Clone)]
pub struct DeleteChatStickerSetRequest<'a> {
    api: &'a API,
    params: DeleteChatStickerSetParams,
}

impl<'a> RequestT for DeleteChatStickerSetRequest<'a> {
    type ParamsType = DeleteChatStickerSetParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "deleteChatStickerSet"
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
impl<'a> DeleteChatStickerSetRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId) -> Self {
        Self {
            api,
            params: DeleteChatStickerSetParams { chat_id },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }
}

impl<'a> API {
    ///Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\_set\_sticker\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
    pub fn delete_chat_sticker_set(
        &'a self,
        chat_id: impl Into<ChatId>,
    ) -> DeleteChatStickerSetRequest {
        DeleteChatStickerSetRequest::new(self, chat_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
