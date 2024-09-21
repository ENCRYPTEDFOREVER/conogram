use crate::api::API;
use crate::entities::input_sticker::InputSticker;
use crate::entities::misc::input_file::GetFiles;
use crate::entities::misc::input_file::InputFile;
use crate::entities::misc::input_file::Moose;
use crate::errors::ConogramError;
use crate::impl_into_future_multipart;
use crate::request::RequestT;
use serde::Serialize;
use std::collections::HashMap;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct ReplaceStickerInSetParams {
    pub user_id: i64,
    pub name: String,
    pub old_sticker: String,
    pub sticker: InputSticker,
}

impl GetFiles for ReplaceStickerInSetParams {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        map.extend(self.sticker.get_files());
        map
    }
}
impl_into_future_multipart!(ReplaceStickerInSetRequest<'a>);

///Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling [deleteStickerFromSet](https://core.telegram.org/bots/api/#deletestickerfromset), then [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), then [setStickerPositionInSet](https://core.telegram.org/bots/api/#setstickerpositioninset). Returns *True* on success.
#[derive(Clone)]
pub struct ReplaceStickerInSetRequest<'a> {
    api: &'a API,
    params: ReplaceStickerInSetParams,
}

impl<'a> RequestT for ReplaceStickerInSetRequest<'a> {
    type ParamsType = ReplaceStickerInSetParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "replaceStickerInSet"
    }
    fn get_api_ref(&self) -> &API {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        true
    }
}
impl<'a> ReplaceStickerInSetRequest<'a> {
    pub fn new(
        api: &'a API,
        user_id: impl Into<i64>,
        name: impl Into<String>,
        old_sticker: impl Into<String>,
        sticker: impl Into<InputSticker>,
    ) -> Self {
        Self {
            api,
            params: ReplaceStickerInSetParams {
                user_id: user_id.into(),
                name: name.into(),
                old_sticker: old_sticker.into(),
                sticker: sticker.into(),
            },
        }
    }

    ///User identifier of the sticker set owner
    #[must_use]
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///Sticker set name
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = name.into();
        self
    }

    ///File identifier of the replaced sticker
    #[must_use]
    pub fn old_sticker(mut self, old_sticker: impl Into<String>) -> Self {
        self.params.old_sticker = old_sticker.into();
        self
    }

    ///A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set remains unchanged.
    #[must_use]
    pub fn sticker(mut self, sticker: impl Into<InputSticker>) -> Self {
        self.params.sticker = sticker.into();
        self
    }
}

impl API {
    ///Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling [deleteStickerFromSet](https://core.telegram.org/bots/api/#deletestickerfromset), then [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), then [setStickerPositionInSet](https://core.telegram.org/bots/api/#setstickerpositioninset). Returns *True* on success.
    pub fn replace_sticker_in_set(
        &self,
        user_id: impl Into<i64>,
        name: impl Into<String>,
        old_sticker: impl Into<String>,
        sticker: impl Into<InputSticker>,
    ) -> ReplaceStickerInSetRequest {
        ReplaceStickerInSetRequest::new(self, user_id, name, old_sticker, sticker)
    }
}

// Divider: all content below this line will be preserved after code regen
