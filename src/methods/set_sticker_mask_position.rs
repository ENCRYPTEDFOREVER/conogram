use crate::api::API;
use crate::entities::mask_position::MaskPosition;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetStickerMaskPositionParams {
    pub sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

impl_into_future!(SetStickerMaskPositionRequest<'a>);

///Use this method to change the [mask position](https://core.telegram.org/bots/api/#maskposition) of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns *True* on success.
#[derive(Clone)]
pub struct SetStickerMaskPositionRequest<'a> {
    api: &'a API,
    params: SetStickerMaskPositionParams,
}

impl<'a> RequestT for SetStickerMaskPositionRequest<'a> {
    type ParamsType = SetStickerMaskPositionParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setStickerMaskPosition"
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
impl<'a> SetStickerMaskPositionRequest<'a> {
    pub fn new(api: &'a API, sticker: impl Into<String>) -> Self {
        Self {
            api,
            params: SetStickerMaskPositionParams {
                sticker: sticker.into(),
                mask_position: Option::default(),
            },
        }
    }

    ///File identifier of the sticker
    pub fn sticker(mut self, sticker: impl Into<String>) -> Self {
        self.params.sticker = sticker.into();
        self
    }

    ///A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.
    pub fn mask_position(mut self, mask_position: impl Into<MaskPosition>) -> Self {
        self.params.mask_position = Some(mask_position.into());
        self
    }
}

impl<'a> API {
    ///Use this method to change the [mask position](https://core.telegram.org/bots/api/#maskposition) of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns *True* on success.
    pub fn set_sticker_mask_position(
        &'a self,
        sticker: impl Into<String>,
    ) -> SetStickerMaskPositionRequest {
        SetStickerMaskPositionRequest::new(self, sticker)
    }
}

// Divider: all content below this line will be preserved after code regen
