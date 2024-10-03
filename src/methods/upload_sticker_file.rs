use std::{
    collections::HashMap,
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API,
    entities::{
        file::File,
        misc::input_file::{GetFiles, InputFile, Moose},
    },
    errors::ConogramError,
    impl_into_future_multipart,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct UploadStickerFileParams {
    pub user_id: i64,
    #[serde(skip)]
    pub sticker: InputFile,
    pub sticker_format: UploadStickerFileStickerFormat,
}

impl GetFiles for UploadStickerFileParams {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        map.insert(Moose::Owned("sticker".into()), &self.sticker);
        map
    }
}
impl_into_future_multipart!(UploadStickerFileRequest<'a>);

///Use this method to upload a file with a sticker for later use in the [createNewStickerSet](https://core.telegram.org/bots/api/#createnewstickerset), [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), or [replaceStickerInSet](https://core.telegram.org/bots/api/#replacestickerinset) methods (the file can be used multiple times). Returns the uploaded [File](https://core.telegram.org/bots/api/#file) on success.
#[derive(Clone)]
pub struct UploadStickerFileRequest<'a> {
    api: &'a API,
    params: UploadStickerFileParams,
}

impl<'a> RequestT for UploadStickerFileRequest<'a> {
    type ParamsType = UploadStickerFileParams;
    type ReturnType = File;
    fn get_name() -> &'static str {
        "uploadStickerFile"
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
impl<'a> UploadStickerFileRequest<'a> {
    pub fn new(
        api: &'a API,
        user_id: impl Into<i64>,
        sticker: impl Into<InputFile>,
        sticker_format: impl Into<UploadStickerFileStickerFormat>,
    ) -> Self {
        Self {
            api,
            params: UploadStickerFileParams {
                user_id: user_id.into(),
                sticker: sticker.into(),
                sticker_format: sticker_format.into(),
            },
        }
    }

    ///User identifier of sticker file owner
    #[must_use]
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///A file with the sticker in .WEBP, .PNG, .TGS, or .WEBM format. See [https://core.telegram.org/stickers](https://core.telegram.org/stickers) for technical requirements. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    #[must_use]
    pub fn sticker(mut self, sticker: impl Into<InputFile>) -> Self {
        self.params.sticker = sticker.into();
        self
    }

    ///Format of the sticker, must be one of “static”, “animated”, “video”
    #[must_use]
    pub fn sticker_format(
        mut self,
        sticker_format: impl Into<UploadStickerFileStickerFormat>,
    ) -> Self {
        self.params.sticker_format = sticker_format.into();
        self
    }
}

impl API {
    ///Use this method to upload a file with a sticker for later use in the [createNewStickerSet](https://core.telegram.org/bots/api/#createnewstickerset), [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), or [replaceStickerInSet](https://core.telegram.org/bots/api/#replacestickerinset) methods (the file can be used multiple times). Returns the uploaded [File](https://core.telegram.org/bots/api/#file) on success.
    pub fn upload_sticker_file(
        &self,
        user_id: impl Into<i64>,
        sticker: impl Into<InputFile>,
        sticker_format: impl Into<UploadStickerFileStickerFormat>,
    ) -> UploadStickerFileRequest {
        UploadStickerFileRequest::new(self, user_id, sticker, sticker_format)
    }
}

///Format of the sticker, must be one of “static”, “animated”, “video”
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "sticker_format")]
pub enum UploadStickerFileStickerFormat {
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
