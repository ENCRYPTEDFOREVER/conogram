use std::{
    collections::HashMap,
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API,
    entities::{
        input_sticker::InputSticker,
        misc::input_file::{GetFiles, InputFile, Moose},
    },
    errors::ConogramError,
    impl_into_future_multipart,
    request::RequestT,
};

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

///Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns *True* on success.
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
    pub fn new(
        api: &'a API,
        user_id: impl Into<i64>,
        name: impl Into<String>,
        sticker: impl Into<InputSticker>,
    ) -> Self {
        Self {
            api,
            params: AddStickerToSetParams {
                user_id: user_id.into(),
                name: name.into(),
                sticker: sticker.into(),
            },
        }
    }

    ///User identifier of sticker set owner
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

    ///A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed.
    #[must_use]
    pub fn sticker(mut self, sticker: impl Into<InputSticker>) -> Self {
        self.params.sticker = sticker.into();
        self
    }
}

impl API {
    ///Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns *True* on success.
    pub fn add_sticker_to_set(
        &self,
        user_id: impl Into<i64>,
        name: impl Into<String>,
        sticker: impl Into<InputSticker>,
    ) -> AddStickerToSetRequest {
        AddStickerToSetRequest::new(self, user_id, name, sticker)
    }
}

// Divider: all content below this line will be preserved after code regen
