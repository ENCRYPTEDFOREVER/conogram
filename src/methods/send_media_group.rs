use crate::api::API;
use crate::entities::input_media::InputMedia;
use crate::entities::message::Message;
use crate::entities::misc::chat_id::ChatId;
use crate::entities::misc::input_file::GetFiles;
use crate::entities::misc::input_file::InputFile;
use crate::entities::misc::input_file::Moose;
use crate::entities::reply_parameters::ReplyParameters;
use crate::errors::ConogramError;
use crate::impl_into_future_multipart;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::collections::HashMap;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SendMediaGroupParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub media: Vec<InputMedia>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_notification: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub protect_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
}

impl GetFiles for SendMediaGroupParams {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        for media in self.media.iter() {
            map.extend(media.get_files());
        }
        map
    }
}
impl_into_future_multipart!(SendMediaGroupRequest<'a>);

///Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of [Messages](https://core.telegram.org/bots/api/#message) that were sent is returned.
#[derive(Clone)]
pub struct SendMediaGroupRequest<'a> {
    api: &'a API,
    params: SendMediaGroupParams,
}

impl<'a> RequestT for SendMediaGroupRequest<'a> {
    type ParamsType = SendMediaGroupParams;
    type ReturnType = Vec<Message>;
    fn get_name() -> &'static str {
        "sendMediaGroup"
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
impl<'a> SendMediaGroupRequest<'a> {
    pub fn new(
        api: &'a API,
        chat_id: impl Into<ChatId>,
        media: impl Into<Vec<InputMedia>>,
    ) -> Self {
        Self {
            api,
            params: SendMediaGroupParams {
                chat_id: chat_id.into(),
                media: media.into(),
                message_thread_id: Option::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
                reply_parameters: Option::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
        self.params.message_thread_id = Some(message_thread_id.into());
        self
    }

    ///A JSON-serialized array describing messages to be sent, must include 2-10 items
    pub fn media(mut self, media: impl IntoIterator<Item = impl Into<InputMedia>>) -> Self {
        self.params.media = media.into_iter().map(Into::into).collect();
        self
    }

    ///Sends messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the sent messages from forwarding and saving
    pub fn protect_content(mut self, protect_content: impl Into<bool>) -> Self {
        self.params.protect_content = protect_content.into();
        self
    }

    ///Description of the message to reply to
    pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
        self.params.reply_parameters = Some(reply_parameters.into());
        self
    }
}

impl<'a> API {
    ///Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of [Messages](https://core.telegram.org/bots/api/#message) that were sent is returned.
    pub fn send_media_group(
        &'a self,
        chat_id: impl Into<ChatId>,
        media: impl Into<Vec<InputMedia>>,
    ) -> SendMediaGroupRequest {
        SendMediaGroupRequest::new(self, chat_id.into(), media.into())
    }
}

// Divider: all content below this line will be preserved after code regen
