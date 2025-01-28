use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{api::Api, errors::ConogramError, impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]
pub struct SetStickerSetTitleParams {
    pub name: String,
    pub title: String,
}

impl_into_future!(SetStickerSetTitleRequest<'a>);

///Use this method to set the title of a created sticker set. Returns *True* on success.
#[derive(Clone)]
pub struct SetStickerSetTitleRequest<'a> {
    api: &'a Api,
    params: SetStickerSetTitleParams,
}

impl RequestT for SetStickerSetTitleRequest<'_> {
    type ParamsType = SetStickerSetTitleParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setStickerSetTitle"
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
impl<'a> SetStickerSetTitleRequest<'a> {
    pub fn new(api: &'a Api, name: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            api,
            params: SetStickerSetTitleParams {
                name: name.into(),
                title: title.into(),
            },
        }
    }

    ///Sticker set name
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = name.into();
        self
    }

    ///Sticker set title, 1-64 characters
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.params.title = title.into();
        self
    }
}

impl Api {
    ///Use this method to set the title of a created sticker set. Returns *True* on success.
    pub fn set_sticker_set_title(
        &self,
        name: impl Into<String>,
        title: impl Into<String>,
    ) -> SetStickerSetTitleRequest {
        SetStickerSetTitleRequest::new(self, name, title)
    }
}

// Divider: all content below this line will be preserved after code regen
