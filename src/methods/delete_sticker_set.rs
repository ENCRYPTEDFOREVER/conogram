


use serde::Serialize;

use crate::{api::Api,  impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]

pub struct DeleteStickerSetParams {
    pub name: String,
}

impl_into_future!(DeleteStickerSetRequest<'a>);

///Use this method to delete a sticker set that was created by the bot. Returns *True* on success.
#[derive(Clone)]
pub struct DeleteStickerSetRequest<'a> {
    api: &'a Api,
    params: DeleteStickerSetParams,
}

impl RequestT for DeleteStickerSetRequest<'_> {
    type ParamsType = DeleteStickerSetParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "deleteStickerSet"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> DeleteStickerSetRequest<'a> {
    pub fn new(api: &'a Api, name: impl Into<String>) -> Self {
        Self {
            api,
            params: DeleteStickerSetParams { name: name.into() },
        }
    }

    ///Sticker set name
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = name.into();
        self
    }
}

impl Api {
    ///Use this method to delete a sticker set that was created by the bot. Returns *True* on success.
    pub fn delete_sticker_set(&self, name: impl Into<String>) -> DeleteStickerSetRequest {
        DeleteStickerSetRequest::new(self, name)
    }
}

// Divider: all content below this line will be preserved after code regen
