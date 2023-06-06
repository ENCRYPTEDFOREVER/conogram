use crate::api::API;
use crate::entities::input_sticker::InputSticker;
use crate::entities::misc::input_file::GetFiles;
use crate::entities::misc::input_file::InputFile;
use crate::entities::misc::input_file::Moose;
use crate::errors::Error;
use crate::impl_into_future_multipart;
use crate::request::RequestT;
use serde::Serialize;
use std::collections::HashMap;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct AddStickerToSetParams {
    pub user_id: i64,
    pub name: String,
    pub sticker: InputSticker,
}

impl GetFiles for AddStickerToSetParams {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        map.extend(self.sticker.get_files());
        map
    }
}
impl_into_future_multipart!(AddStickerToSetRequest<'a>);

///Use this method to add a new sticker to a set created by the bot. The format of the added sticker must match the format of the other stickers in the set. Emoji sticker sets can have up to 200 stickers. Animated and video sticker sets can have up to 50 stickers. Static sticker sets can have up to 120 stickers. Returns *True* on success.
#[derive(Clone)]
pub struct AddStickerToSetRequest<'a> {
    api: &'a API,
    params: AddStickerToSetParams,
}

impl<'a> RequestT for AddStickerToSetRequest<'a> {
    type ParamsType = AddStickerToSetParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "addStickerToSet"
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
impl<'a> AddStickerToSetRequest<'a> {
    pub fn new(api: &'a API, user_id: i64, name: String, sticker: InputSticker) -> Self {
        Self {
            api,
            params: AddStickerToSetParams {
                user_id,
                name,
                sticker,
            },
        }
    }

    ///User identifier of sticker set owner
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///Sticker set name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = name.into();
        self
    }

    ///A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed.
    pub fn sticker(mut self, sticker: impl Into<InputSticker>) -> Self {
        self.params.sticker = sticker.into();
        self
    }
}

impl<'a> API {
    ///Use this method to add a new sticker to a set created by the bot. The format of the added sticker must match the format of the other stickers in the set. Emoji sticker sets can have up to 200 stickers. Animated and video sticker sets can have up to 50 stickers. Static sticker sets can have up to 120 stickers. Returns *True* on success.
    pub fn add_sticker_to_set(
        &'a self,
        user_id: impl Into<i64>,
        name: impl Into<String>,
        sticker: impl Into<InputSticker>,
    ) -> AddStickerToSetRequest {
        AddStickerToSetRequest::new(self, user_id.into(), name.into(), sticker.into())
    }
}

// Divider: all content below this line will be preserved after code regen
