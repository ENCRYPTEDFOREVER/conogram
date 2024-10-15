use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{api::API, errors::ConogramError, impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]
pub struct SetStickerPositionInSetParams {
    pub sticker: String,
    pub position: i64,
}

impl_into_future!(SetStickerPositionInSetRequest<'a>);

///Use this method to move a sticker in a set created by the bot to a specific position. Returns *True* on success.
#[derive(Clone)]
pub struct SetStickerPositionInSetRequest<'a> {
    api: &'a API,
    params: SetStickerPositionInSetParams,
}

impl<'a> RequestT for SetStickerPositionInSetRequest<'a> {
    type ParamsType = SetStickerPositionInSetParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setStickerPositionInSet"
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
impl<'a> SetStickerPositionInSetRequest<'a> {
    pub fn new(api: &'a API, sticker: impl Into<String>, position: impl Into<i64>) -> Self {
        Self {
            api,
            params: SetStickerPositionInSetParams {
                sticker: sticker.into(),
                position: position.into(),
            },
        }
    }

    ///File identifier of the sticker
    #[must_use]
    pub fn sticker(mut self, sticker: impl Into<String>) -> Self {
        self.params.sticker = sticker.into();
        self
    }

    ///New sticker position in the set, zero-based
    #[must_use]
    pub fn position(mut self, position: impl Into<i64>) -> Self {
        self.params.position = position.into();
        self
    }
}

impl API {
    ///Use this method to move a sticker in a set created by the bot to a specific position. Returns *True* on success.
    pub fn set_sticker_position_in_set(
        &self,
        sticker: impl Into<String>,
        position: impl Into<i64>,
    ) -> SetStickerPositionInSetRequest {
        SetStickerPositionInSetRequest::new(self, sticker, position)
    }
}

// Divider: all content below this line will be preserved after code regen
