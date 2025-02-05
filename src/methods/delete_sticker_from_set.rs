


use serde::Serialize;

use crate::{api::Api,  impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]

pub struct DeleteStickerFromSetParams {
    pub sticker: String,
}

impl_into_future!(DeleteStickerFromSetRequest<'a>);

///Use this method to delete a sticker from a set created by the bot. Returns *True* on success.
#[derive(Clone)]
pub struct DeleteStickerFromSetRequest<'a> {
    api: &'a Api,
    params: DeleteStickerFromSetParams,
}

impl RequestT for DeleteStickerFromSetRequest<'_> {
    type ParamsType = DeleteStickerFromSetParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "deleteStickerFromSet"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> DeleteStickerFromSetRequest<'a> {
    pub fn new(api: &'a Api, sticker: impl Into<String>) -> Self {
        Self {
            api,
            params: DeleteStickerFromSetParams {
                sticker: sticker.into(),
            },
        }
    }

    ///File identifier of the sticker
    #[must_use]
    pub fn sticker(mut self, sticker: impl Into<String>) -> Self {
        self.params.sticker = sticker.into();
        self
    }
}

impl Api {
    ///Use this method to delete a sticker from a set created by the bot. Returns *True* on success.
    pub fn delete_sticker_from_set(
        &self,
        sticker: impl Into<String>,
    ) -> DeleteStickerFromSetRequest {
        DeleteStickerFromSetRequest::new(self, sticker)
    }
}

// Divider: all content below this line will be preserved after code regen
