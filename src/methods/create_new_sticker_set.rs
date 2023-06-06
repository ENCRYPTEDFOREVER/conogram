use crate::api::API;
use crate::entities::input_sticker::InputSticker;
use crate::entities::misc::input_file::GetFiles;
use crate::entities::misc::input_file::InputFile;
use crate::entities::misc::input_file::Moose;
use crate::errors::Error;
use crate::impl_into_future_multipart;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::collections::HashMap;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct CreateNewStickerSetParams {
    pub user_id: i64,
    pub name: String,
    pub title: String,
    pub stickers: Vec<InputSticker>,
    pub sticker_format: CreateNewStickerSetStickerFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<CreateNewStickerSetStickerType>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub needs_repainting: bool,
}

impl GetFiles for CreateNewStickerSetParams {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        for stickers in self.stickers.iter() {
            map.extend(stickers.get_files());
        }
        map
    }
}
impl_into_future_multipart!(CreateNewStickerSetRequest<'a>);

///Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns *True* on success.
#[derive(Clone)]
pub struct CreateNewStickerSetRequest<'a> {
    api: &'a API,
    params: CreateNewStickerSetParams,
}

impl<'a> RequestT for CreateNewStickerSetRequest<'a> {
    type ParamsType = CreateNewStickerSetParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "createNewStickerSet"
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
impl<'a> CreateNewStickerSetRequest<'a> {
    pub fn new(
        api: &'a API,
        user_id: i64,
        name: String,
        title: String,
        stickers: Vec<InputSticker>,
        sticker_format: CreateNewStickerSetStickerFormat,
    ) -> Self {
        Self {
            api,
            params: CreateNewStickerSetParams {
                user_id,
                name,
                title,
                stickers,
                sticker_format,
                sticker_type: Option::default(),
                needs_repainting: bool::default(),
            },
        }
    }

    ///User identifier of created sticker set owner
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g., *animals*). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in `"_by_<bot_username>"`. `<bot_username>` is case insensitive. 1-64 characters.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = name.into();
        self
    }

    ///Sticker set title, 1-64 characters
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.params.title = title.into();
        self
    }

    ///A JSON-serialized list of 1-50 initial stickers to be added to the sticker set
    pub fn stickers(mut self, stickers: impl Into<Vec<InputSticker>>) -> Self {
        self.params.stickers = stickers.into();
        self
    }

    ///Format of stickers in the set, must be one of “static”, “animated”, “video”
    pub fn sticker_format(
        mut self,
        sticker_format: impl Into<CreateNewStickerSetStickerFormat>,
    ) -> Self {
        self.params.sticker_format = sticker_format.into();
        self
    }

    ///Type of stickers in the set, pass “regular”, “mask”, or “custom\_emoji”. By default, a regular sticker set is created.
    pub fn sticker_type(mut self, sticker_type: impl Into<CreateNewStickerSetStickerType>) -> Self {
        self.params.sticker_type = Some(sticker_type.into());
        self
    }

    ///Pass *True* if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only
    pub fn needs_repainting(mut self, needs_repainting: impl Into<bool>) -> Self {
        self.params.needs_repainting = needs_repainting.into();
        self
    }
}

impl<'a> API {
    ///Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns *True* on success.
    pub fn create_new_sticker_set(
        &'a self,
        user_id: impl Into<i64>,
        name: impl Into<String>,
        title: impl Into<String>,
        stickers: impl Into<Vec<InputSticker>>,
        sticker_format: impl Into<CreateNewStickerSetStickerFormat>,
    ) -> CreateNewStickerSetRequest {
        CreateNewStickerSetRequest::new(
            self,
            user_id.into(),
            name.into(),
            title.into(),
            stickers.into(),
            sticker_format.into(),
        )
    }
}

///Format of stickers in the set, must be one of “static”, “animated”, “video”
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "sticker_format")]
pub enum CreateNewStickerSetStickerFormat {
    #[default]
    /// "static"
    #[serde(rename = "static")]
    Static,

    /// "animated"
    #[serde(rename = "animated")]
    Animated,

    /// "video"
    #[serde(rename = "video")]
    Video,
}

///Type of stickers in the set, pass “regular”, “mask”, or “custom\_emoji”. By default, a regular sticker set is created.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "sticker_type")]
pub enum CreateNewStickerSetStickerType {
    #[default]
    /// "mask"
    #[serde(rename = "mask")]
    Mask,

    /// "custom_emoji"
    #[serde(rename = "custom_emoji")]
    CustomEmoji,
}
// Divider: all content below this line will be preserved after code regen
