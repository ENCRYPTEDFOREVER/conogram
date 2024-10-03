use std::{
    collections::HashMap,
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API,
    entities::misc::{
        chat_id::ChatId,
        input_file::{GetFiles, InputFile, Moose},
    },
    errors::ConogramError,
    impl_into_future_multipart,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct SetChatPhotoParams {
    pub chat_id: ChatId,
    #[serde(skip)]
    pub photo: InputFile,
}

impl GetFiles for SetChatPhotoParams {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        map.insert(Moose::Owned("photo".into()), &self.photo);
        map
    }
}
impl_into_future_multipart!(SetChatPhotoRequest<'a>);

///Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
#[derive(Clone)]
pub struct SetChatPhotoRequest<'a> {
    api: &'a API,
    params: SetChatPhotoParams,
}

impl<'a> RequestT for SetChatPhotoRequest<'a> {
    type ParamsType = SetChatPhotoParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setChatPhoto"
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
impl<'a> SetChatPhotoRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, photo: impl Into<InputFile>) -> Self {
        Self {
            api,
            params: SetChatPhotoParams {
                chat_id: chat_id.into(),
                photo: photo.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///New chat photo, uploaded using multipart/form-data
    #[must_use]
    pub fn photo(mut self, photo: impl Into<InputFile>) -> Self {
        self.params.photo = photo.into();
        self
    }
}

impl API {
    ///Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    pub fn set_chat_photo(
        &self,
        chat_id: impl Into<ChatId>,
        photo: impl Into<InputFile>,
    ) -> SetChatPhotoRequest {
        SetChatPhotoRequest::new(self, chat_id, photo)
    }
}

// Divider: all content below this line will be preserved after code regen
