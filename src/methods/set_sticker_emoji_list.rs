use crate::api::API;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetStickerEmojiListParams {
    pub sticker: String,
    pub emoji_list: Vec<String>,
}

impl_into_future!(SetStickerEmojiListRequest<'a>);

///Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
#[derive(Clone)]
pub struct SetStickerEmojiListRequest<'a> {
    api: &'a API,
    params: SetStickerEmojiListParams,
}

impl<'a> RequestT for SetStickerEmojiListRequest<'a> {
    type ParamsType = SetStickerEmojiListParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setStickerEmojiList"
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
impl<'a> SetStickerEmojiListRequest<'a> {
    pub fn new(api: &'a API, sticker: String, emoji_list: Vec<String>) -> Self {
        Self {
            api,
            params: SetStickerEmojiListParams {
                sticker,
                emoji_list,
            },
        }
    }

    ///File identifier of the sticker
    pub fn sticker(mut self, sticker: impl Into<String>) -> Self {
        self.params.sticker = sticker.into();
        self
    }

    ///A JSON-serialized list of 1-20 emoji associated with the sticker
    pub fn emoji_list(mut self, emoji_list: impl Into<Vec<String>>) -> Self {
        self.params.emoji_list = emoji_list.into();
        self
    }
}

impl<'a> API {
    ///Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
    pub fn set_sticker_emoji_list(
        &'a self,
        sticker: impl Into<String>,
        emoji_list: impl Into<Vec<String>>,
    ) -> SetStickerEmojiListRequest {
        SetStickerEmojiListRequest::new(self, sticker.into(), emoji_list.into())
    }
}

// Divider: all content below this line will be preserved after code regen
