use crate::api::API;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetCustomEmojiStickerSetThumbnailParams {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}

impl_into_future!(SetCustomEmojiStickerSetThumbnailRequest<'a>);

///Use this method to set the thumbnail of a custom emoji sticker set. Returns *True* on success.
#[derive(Clone)]
pub struct SetCustomEmojiStickerSetThumbnailRequest<'a> {
    api: &'a API,
    params: SetCustomEmojiStickerSetThumbnailParams,
}

impl<'a> RequestT for SetCustomEmojiStickerSetThumbnailRequest<'a> {
    type ParamsType = SetCustomEmojiStickerSetThumbnailParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setCustomEmojiStickerSetThumbnail"
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
impl<'a> SetCustomEmojiStickerSetThumbnailRequest<'a> {
    pub fn new(api: &'a API, name: impl Into<String>) -> Self {
        Self {
            api,
            params: SetCustomEmojiStickerSetThumbnailParams {
                name: name.into(),
                custom_emoji_id: Option::default(),
            },
        }
    }

    ///Sticker set name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = name.into();
        self
    }

    ///Custom emoji identifier of a sticker from the sticker set; pass an empty string to drop the thumbnail and use the first sticker as the thumbnail.
    pub fn custom_emoji_id(mut self, custom_emoji_id: impl Into<String>) -> Self {
        self.params.custom_emoji_id = Some(custom_emoji_id.into());
        self
    }
}

impl<'a> API {
    ///Use this method to set the thumbnail of a custom emoji sticker set. Returns *True* on success.
    pub fn set_custom_emoji_sticker_set_thumbnail(
        &'a self,
        name: impl Into<String>,
    ) -> SetCustomEmojiStickerSetThumbnailRequest {
        SetCustomEmojiStickerSetThumbnailRequest::new(self, name.into())
    }
}

// Divider: all content below this line will be preserved after code regen
