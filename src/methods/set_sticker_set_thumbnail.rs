use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api,
    entities::misc::input_file::{GetFiles, InputFile},
    errors::ConogramError,
    impl_into_future_multipart,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct SetStickerSetThumbnailParams {
    pub name: String,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,
    pub format: SetStickerSetThumbnailFormat,
}

impl GetFiles for SetStickerSetThumbnailParams {
    fn get_files(&self) -> Vec<&InputFile> {
        let mut vec = Vec::with_capacity(3);
        if let Some(thumbnail) = &self.thumbnail {
            vec.push(thumbnail);
        }
        vec
    }
}
impl_into_future_multipart!(SetStickerSetThumbnailRequest<'a>);

///Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns *True* on success.
#[derive(Clone)]
pub struct SetStickerSetThumbnailRequest<'a> {
    api: &'a Api,
    params: SetStickerSetThumbnailParams,
}

impl RequestT for SetStickerSetThumbnailRequest<'_> {
    type ParamsType = SetStickerSetThumbnailParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setStickerSetThumbnail"
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
impl<'a> SetStickerSetThumbnailRequest<'a> {
    pub fn new(
        api: &'a Api,
        name: impl Into<String>,
        user_id: impl Into<i64>,
        format: impl Into<SetStickerSetThumbnailFormat>,
    ) -> Self {
        Self {
            api,
            params: SetStickerSetThumbnailParams {
                name: name.into(),
                user_id: user_id.into(),
                format: format.into(),
                thumbnail: Option::default(),
            },
        }
    }

    ///Sticker set name
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = name.into();
        self
    }

    ///User identifier of the sticker set owner
    #[must_use]
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///A **.WEBP** or **.PNG** image with the thumbnail, must be up to 128 kilobytes in size and have a width and height of exactly 100px, or a **.TGS** animation with a thumbnail up to 32 kilobytes in size (see [https://core.telegram.org/stickers#animation-requirements](https://core.telegram.org/stickers#animation-requirements) for animated sticker technical requirements), or a **WEBM** video with the thumbnail up to 32 kilobytes in size; see [https://core.telegram.org/stickers#video-requirements](https://core.telegram.org/stickers#video-requirements) for video sticker technical requirements. Pass a *file\_id* as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Animated and video sticker set thumbnails can't be uploaded via HTTP URL. If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail.
    #[must_use]
    pub fn thumbnail(mut self, thumbnail: impl Into<InputFile>) -> Self {
        self.params.thumbnail = Some(thumbnail.into());
        self
    }

    ///Format of the thumbnail, must be one of “static” for a **.WEBP** or **.PNG** image, “animated” for a **.TGS** animation, or “video” for a **WEBM** video
    #[must_use]
    pub fn format(mut self, format: impl Into<SetStickerSetThumbnailFormat>) -> Self {
        self.params.format = format.into();
        self
    }
}

impl Api {
    ///Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns *True* on success.
    pub fn set_sticker_set_thumbnail(
        &self,
        name: impl Into<String>,
        user_id: impl Into<i64>,
        format: impl Into<SetStickerSetThumbnailFormat>,
    ) -> SetStickerSetThumbnailRequest {
        SetStickerSetThumbnailRequest::new(self, name, user_id, format)
    }
}

///Format of the thumbnail, must be one of “static” for a **.WEBP** or **.PNG** image, “animated” for a **.TGS** animation, or “video” for a **WEBM** video
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename = "format")]
pub enum SetStickerSetThumbnailFormat {
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

// Divider: all content below this line will be preserved after code regen
