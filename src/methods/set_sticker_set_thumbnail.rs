use crate::api::API;
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
pub struct SetStickerSetThumbnailParams {
    pub name: String,
    pub user_id: i64,
    #[serde(skip, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,
}

impl GetFiles for SetStickerSetThumbnailParams {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        if let Some(thumbnail) = &self.thumbnail {
            map.insert(Moose::Owned("thumbnail".into()), thumbnail);
        }
        map
    }
}
impl_into_future_multipart!(SetStickerSetThumbnailRequest<'a>);

///Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns *True* on success.
#[derive(Clone)]
pub struct SetStickerSetThumbnailRequest<'a> {
    api: &'a API,
    params: SetStickerSetThumbnailParams,
}

impl<'a> RequestT for SetStickerSetThumbnailRequest<'a> {
    type ParamsType = SetStickerSetThumbnailParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setStickerSetThumbnail"
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
impl<'a> SetStickerSetThumbnailRequest<'a> {
    pub fn new(api: &'a API, name: String, user_id: i64) -> Self {
        Self {
            api,
            params: SetStickerSetThumbnailParams {
                name,
                user_id,
                thumbnail: Option::default(),
            },
        }
    }

    ///Sticker set name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = name.into();
        self
    }

    ///User identifier of the sticker set owner
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///A **.WEBP** or **.PNG** image with the thumbnail, must be up to 128 kilobytes in size and have a width and height of exactly 100px, or a **.TGS** animation with a thumbnail up to 32 kilobytes in size (see [https://core.telegram.org/stickers#animated-sticker-requirements](https://core.telegram.org/stickers#animated-sticker-requirements) for animated sticker technical requirements), or a **WEBM** video with the thumbnail up to 32 kilobytes in size; see [https://core.telegram.org/stickers#video-sticker-requirements](https://core.telegram.org/stickers#video-sticker-requirements) for video sticker technical requirements. Pass a *file\_id* as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Animated and video sticker set thumbnails can't be uploaded via HTTP URL. If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail.
    pub fn thumbnail(mut self, thumbnail: impl Into<InputFile>) -> Self {
        self.params.thumbnail = Some(thumbnail.into());
        self
    }
}

impl<'a> API {
    ///Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns *True* on success.
    pub fn set_sticker_set_thumbnail(
        &'a self,
        name: impl Into<String>,
        user_id: impl Into<i64>,
    ) -> SetStickerSetThumbnailRequest {
        SetStickerSetThumbnailRequest::new(self, name.into(), user_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
