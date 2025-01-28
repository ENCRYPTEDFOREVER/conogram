use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{api::Api, errors::ConogramError, impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]
pub struct SetStickerKeywordsParams {
    pub sticker: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
}

impl_into_future!(SetStickerKeywordsRequest<'a>);

///Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
#[derive(Clone)]
pub struct SetStickerKeywordsRequest<'a> {
    api: &'a Api,
    params: SetStickerKeywordsParams,
}

impl RequestT for SetStickerKeywordsRequest<'_> {
    type ParamsType = SetStickerKeywordsParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setStickerKeywords"
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
impl<'a> SetStickerKeywordsRequest<'a> {
    pub fn new(api: &'a Api, sticker: impl Into<String>) -> Self {
        Self {
            api,
            params: SetStickerKeywordsParams {
                sticker: sticker.into(),
                keywords: Vec::default(),
            },
        }
    }

    ///File identifier of the sticker
    #[must_use]
    pub fn sticker(mut self, sticker: impl Into<String>) -> Self {
        self.params.sticker = sticker.into();
        self
    }

    ///A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters
    #[must_use]
    pub fn keywords(mut self, keywords: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.params.keywords = keywords.into_iter().map(Into::into).collect();
        self
    }
}

impl Api {
    ///Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.
    pub fn set_sticker_keywords(&self, sticker: impl Into<String>) -> SetStickerKeywordsRequest {
        SetStickerKeywordsRequest::new(self, sticker)
    }
}

// Divider: all content below this line will be preserved after code regen
