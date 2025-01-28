use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api, entities::sticker_set::StickerSet, errors::ConogramError, impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetStickerSetParams {
    pub name: String,
}

impl_into_future!(GetStickerSetRequest<'a>);

///Use this method to get a sticker set. On success, a [StickerSet](https://core.telegram.org/bots/api/#stickerset) object is returned.
#[derive(Clone)]
pub struct GetStickerSetRequest<'a> {
    api: &'a Api,
    params: GetStickerSetParams,
}

impl RequestT for GetStickerSetRequest<'_> {
    type ParamsType = GetStickerSetParams;
    type ReturnType = StickerSet;
    fn get_name() -> &'static str {
        "getStickerSet"
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
impl<'a> GetStickerSetRequest<'a> {
    pub fn new(api: &'a Api, name: impl Into<String>) -> Self {
        Self {
            api,
            params: GetStickerSetParams { name: name.into() },
        }
    }

    ///Name of the sticker set
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = name.into();
        self
    }
}

impl Api {
    ///Use this method to get a sticker set. On success, a [StickerSet](https://core.telegram.org/bots/api/#stickerset) object is returned.
    pub fn get_sticker_set(&self, name: impl Into<String>) -> GetStickerSetRequest {
        GetStickerSetRequest::new(self, name)
    }
}

// Divider: all content below this line will be preserved after code regen
