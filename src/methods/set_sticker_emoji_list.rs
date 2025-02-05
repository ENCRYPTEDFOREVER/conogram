


use serde::Serialize;

use crate::{api::Api,  impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]

pub struct SetStickerEmojiListParams {
    pub sticker: String,
    pub emoji_list: Vec<String>,
}

impl_into_future!(SetStickerEmojiListRequest<'a>);

///Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
#[derive(Clone)]
pub struct SetStickerEmojiListRequest<'a> {
    api: &'a Api,
    params: SetStickerEmojiListParams,
}

impl RequestT for SetStickerEmojiListRequest<'_> {
    type ParamsType = SetStickerEmojiListParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setStickerEmojiList"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> SetStickerEmojiListRequest<'a> {
    pub fn new(
        api: &'a Api,
        sticker: impl Into<String>,
        emoji_list: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            api,
            params: SetStickerEmojiListParams {
                sticker: sticker.into(),
                emoji_list: emoji_list.into_iter().map(Into::into).collect(),
            },
        }
    }

    ///File identifier of the sticker
    #[must_use]
    pub fn sticker(mut self, sticker: impl Into<String>) -> Self {
        self.params.sticker = sticker.into();
        self
    }

    ///A JSON-serialized list of 1-20 emoji associated with the sticker
    #[must_use]
    pub fn emoji_list(mut self, emoji_list: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.params.emoji_list = emoji_list.into_iter().map(Into::into).collect();
        self
    }
}

impl Api {
    ///Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
    pub fn set_sticker_emoji_list(
        &self,
        sticker: impl Into<String>,
        emoji_list: impl IntoIterator<Item = impl Into<String>>,
    ) -> SetStickerEmojiListRequest {
        SetStickerEmojiListRequest::new(self, sticker, emoji_list)
    }
}

// Divider: all content below this line will be preserved after code regen
