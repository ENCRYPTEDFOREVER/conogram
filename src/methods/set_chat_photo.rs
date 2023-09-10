use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
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
    pub fn new(api: &'a API, chat_id: ChatId, photo: InputFile) -> Self {
        Self {
            api,
            params: SetChatPhotoParams { chat_id, photo },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///New chat photo, uploaded using multipart/form-data
    pub fn photo(mut self, photo: impl Into<InputFile>) -> Self {
        self.params.photo = photo.into();
        self
    }
}

impl<'a> API {
    ///Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    pub fn set_chat_photo(
        &'a self,
        chat_id: impl Into<ChatId>,
        photo: impl Into<InputFile>,
    ) -> SetChatPhotoRequest {
        SetChatPhotoRequest::new(self, chat_id.into(), photo.into())
    }
}

// Divider: all content below this line will be preserved after code regen
