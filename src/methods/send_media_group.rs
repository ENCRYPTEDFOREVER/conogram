use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API,
    entities::{
        input_media::InputMedia,
        message::Message,
        misc::{
            chat_id::ChatId,
            input_file::{GetFiles, InputFile},
        },
        reply_parameters::ReplyParameters,
    },
    errors::ConogramError,
    impl_into_future_multipart,
    request::RequestT,
    utils::deserialize_utils::is_false,
};

#[derive(Debug, Clone, Serialize)]
pub struct SendMediaGroupParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub media: Vec<InputMedia>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_notification: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub protect_content: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_paid_broadcast: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
}

impl GetFiles for SendMediaGroupParams {
    fn get_files(&self) -> Vec<&InputFile> {
        let mut vec = Vec::with_capacity(3);
        for media in &self.media {
            vec.extend(media.get_files());
        }
        vec
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
        media: impl IntoIterator<Item = impl Into<InputMedia>>,
    ) -> Self {
        Self {
            api,
            params: SendMediaGroupParams {
                chat_id: chat_id.into(),
                media: media.into_iter().map(Into::into).collect(),
                business_connection_id: Option::default(),
                message_thread_id: Option::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
                allow_paid_broadcast: bool::default(),
                message_effect_id: Option::default(),
                reply_parameters: Option::default(),
            },
        }
    }

    ///Unique identifier of the business connection on behalf of which the message will be sent
    #[must_use]
    pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(business_connection_id.into());
        self
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[must_use]
    pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
        self.params.message_thread_id = Some(message_thread_id.into());
        self
    }

    ///A JSON-serialized array describing messages to be sent, must include 2-10 items
    #[must_use]
    pub fn media(mut self, media: impl IntoIterator<Item = impl Into<InputMedia>>) -> Self {
        self.params.media = media.into_iter().map(Into::into).collect();
        self
    }

    ///Sends messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the sent messages from forwarding and saving
    #[must_use]
    pub fn protect_content(mut self, protect_content: impl Into<bool>) -> Self {
        self.params.protect_content = protect_content.into();
        self
    }

    ///Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[must_use]
    pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: impl Into<bool>) -> Self {
        self.params.allow_paid_broadcast = allow_paid_broadcast.into();
        self
    }

    ///Unique identifier of the message effect to be added to the message; for private chats only
    #[must_use]
    pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(message_effect_id.into());
        self
    }

    ///Description of the message to reply to
    #[must_use]
    pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
        self.params.reply_parameters = Some(reply_parameters.into());
        self
    }
}

impl API {
    ///Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of [Messages](https://core.telegram.org/bots/api/#message) that were sent is returned.
    pub fn send_media_group(
        &self,
        chat_id: impl Into<ChatId>,
        media: impl IntoIterator<Item = impl Into<InputMedia>>,
    ) -> SendMediaGroupRequest {
        SendMediaGroupRequest::new(self, chat_id, media)
    }
}

// Divider: all content below this line will be preserved after code regen
