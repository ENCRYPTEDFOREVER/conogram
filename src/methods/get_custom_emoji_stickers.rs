use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API, entities::sticker::Sticker, errors::ConogramError, impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetCustomEmojiStickersParams {
    pub custom_emoji_ids: Vec<String>,
}

impl_into_future!(GetCustomEmojiStickersRequest<'a>);

///Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
#[derive(Clone)]
pub struct GetCustomEmojiStickersRequest<'a> {
    api: &'a API,
    params: GetCustomEmojiStickersParams,
}

impl<'a> RequestT for GetCustomEmojiStickersRequest<'a> {
    type ParamsType = GetCustomEmojiStickersParams;
    type ReturnType = Vec<Sticker>;
    fn get_name() -> &'static str {
        "getCustomEmojiStickers"
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
impl<'a> GetCustomEmojiStickersRequest<'a> {
    pub fn new(
        api: &'a API,
        custom_emoji_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            api,
            params: GetCustomEmojiStickersParams {
                custom_emoji_ids: custom_emoji_ids.into_iter().map(Into::into).collect(),
            },
        }
    }

    ///A JSON-serialized list of custom emoji identifiers. At most 200 custom emoji identifiers can be specified.
    #[must_use]
    pub fn custom_emoji_ids(
        mut self,
        custom_emoji_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.params.custom_emoji_ids = custom_emoji_ids.into_iter().map(Into::into).collect();
        self
    }
}

impl API {
    ///Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
    pub fn get_custom_emoji_stickers(
        &self,
        custom_emoji_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> GetCustomEmojiStickersRequest {
        GetCustomEmojiStickersRequest::new(self, custom_emoji_ids)
    }
}

// Divider: all content below this line will be preserved after code regen
