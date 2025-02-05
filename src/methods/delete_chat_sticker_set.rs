


use serde::Serialize;

use crate::{
    api::Api, entities::misc::chat_id::ChatId,  impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]

pub struct DeleteChatStickerSetParams {
    pub chat_id: ChatId,
}

impl_into_future!(DeleteChatStickerSetRequest<'a>);

///Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\_set\_sticker\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
#[derive(Clone)]
pub struct DeleteChatStickerSetRequest<'a> {
    api: &'a Api,
    params: DeleteChatStickerSetParams,
}

impl RequestT for DeleteChatStickerSetRequest<'_> {
    type ParamsType = DeleteChatStickerSetParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "deleteChatStickerSet"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> DeleteChatStickerSetRequest<'a> {
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: DeleteChatStickerSetParams {
                chat_id: chat_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }
}

impl Api {
    ///Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\_set\_sticker\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.
    pub fn delete_chat_sticker_set(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> DeleteChatStickerSetRequest {
        DeleteChatStickerSetRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
