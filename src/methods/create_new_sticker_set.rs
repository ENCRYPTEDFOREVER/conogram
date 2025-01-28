use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api,
    entities::{
        input_sticker::InputSticker,
        misc::input_file::{GetFiles, InputFile},
    },
    errors::ConogramError,
    impl_into_future_multipart,
    request::RequestT,
    utils::deserialize_utils::is_false,
};

#[derive(Debug, Clone, Serialize)]
pub struct CreateNewStickerSetParams {
    pub user_id: i64,
    pub name: String,
    pub title: String,
    pub stickers: Vec<InputSticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<CreateNewStickerSetStickerType>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub needs_repainting: bool,
}

impl GetFiles for CreateNewStickerSetParams {
    fn get_files(&self) -> Vec<&InputFile> {
        let mut vec = Vec::with_capacity(3);
        for stickers in &self.stickers {
            vec.extend(stickers.get_files());
        }
        vec
    }
}
impl_into_future_multipart!(CreateNewStickerSetRequest<'a>);

///Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns *True* on success.
#[derive(Clone)]
pub struct CreateNewStickerSetRequest<'a> {
    api: &'a Api,
    params: CreateNewStickerSetParams,
}

impl RequestT for CreateNewStickerSetRequest<'_> {
    type ParamsType = CreateNewStickerSetParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "createNewStickerSet"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        true
    }
}
impl<'a> CreateNewStickerSetRequest<'a> {
    pub fn new(
        api: &'a Api,
        user_id: impl Into<i64>,
        name: impl Into<String>,
        title: impl Into<String>,
        stickers: impl IntoIterator<Item = impl Into<InputSticker>>,
    ) -> Self {
        Self {
            api,
            params: CreateNewStickerSetParams {
                user_id: user_id.into(),
                name: name.into(),
                title: title.into(),
                stickers: stickers.into_iter().map(Into::into).collect(),
                sticker_type: Option::default(),
                needs_repainting: bool::default(),
            },
        }
    }

    ///User identifier of created sticker set owner
    #[must_use]
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g., *animals*). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in `"_by_<bot_username>"`. `<bot_username>` is case insensitive. 1-64 characters.
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

    ///A JSON-serialized list of 1-50 initial stickers to be added to the sticker set
    #[must_use]
    pub fn stickers(mut self, stickers: impl IntoIterator<Item = impl Into<InputSticker>>) -> Self {
        self.params.stickers = stickers.into_iter().map(Into::into).collect();
        self
    }

    ///Type of stickers in the set, pass “regular”, “mask”, or “custom\_emoji”. By default, a regular sticker set is created.
    #[must_use]
    pub fn sticker_type(mut self, sticker_type: impl Into<CreateNewStickerSetStickerType>) -> Self {
        self.params.sticker_type = Some(sticker_type.into());
        self
    }

    ///Pass *True* if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only
    #[must_use]
    pub fn needs_repainting(mut self, needs_repainting: impl Into<bool>) -> Self {
        self.params.needs_repainting = needs_repainting.into();
        self
    }
}

impl Api {
    ///Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns *True* on success.
    pub fn create_new_sticker_set(
        &self,
        user_id: impl Into<i64>,
        name: impl Into<String>,
        title: impl Into<String>,
        stickers: impl IntoIterator<Item = impl Into<InputSticker>>,
    ) -> CreateNewStickerSetRequest {
        CreateNewStickerSetRequest::new(self, user_id, name, title, stickers)
    }
}

///Type of stickers in the set, pass “regular”, “mask”, or “custom\_emoji”. By default, a regular sticker set is created.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename = "sticker_type")]
pub enum CreateNewStickerSetStickerType {
    #[default]
    /// `mask`
    #[serde(rename = "mask")]
    Mask,

    /// `custom_emoji`
    #[serde(rename = "custom_emoji")]
    CustomEmoji,
}

// Divider: all content below this line will be preserved after code regen
