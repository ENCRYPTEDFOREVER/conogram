use std::{
    collections::HashMap,
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API,
    entities::{
        inline_keyboard_markup::InlineKeyboardMarkup,
        input_media::InputMedia,
        message::Message,
        misc::{
            chat_id::ChatId,
            input_file::{GetFiles, InputFile, Moose},
        },
    },
    errors::ConogramError,
    impl_into_future_multipart,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct EditMessageMediaParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub media: InputMedia,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl GetFiles for EditMessageMediaParams {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        map.extend(self.media.get_files());
        map
    }
}
impl_into_future_multipart!(EditMessageMediaRequest<'a>);

///Use this method to edit animation, audio, document, photo, or video messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file\_id or specify a URL. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
#[derive(Clone)]
pub struct EditMessageMediaRequest<'a> {
    api: &'a API,
    params: EditMessageMediaParams,
}

impl<'a> RequestT for EditMessageMediaRequest<'a> {
    type ParamsType = EditMessageMediaParams;
    type ReturnType = Option<Message>;
    fn get_name() -> &'static str {
        "editMessageMedia"
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
impl<'a> EditMessageMediaRequest<'a> {
    pub fn new(api: &'a API, media: impl Into<InputMedia>) -> Self {
        Self {
            api,
            params: EditMessageMediaParams {
                media: media.into(),
                business_connection_id: Option::default(),
                chat_id: Option::default(),
                message_id: Option::default(),
                inline_message_id: Option::default(),
                reply_markup: Option::default(),
            },
        }
    }

    ///Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[must_use]
    pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(business_connection_id.into());
        self
    }

    ///Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = Some(chat_id.into());
        self
    }

    ///Required if *inline\_message\_id* is not specified. Identifier of the message to edit
    #[must_use]
    pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
        self.params.message_id = Some(message_id.into());
        self
    }

    ///Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message
    #[must_use]
    pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(inline_message_id.into());
        self
    }

    ///A JSON-serialized object for a new media content of the message
    #[must_use]
    pub fn media(mut self, media: impl Into<InputMedia>) -> Self {
        self.params.media = media.into();
        self
    }

    ///A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    #[must_use]
    pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl API {
    ///Use this method to edit animation, audio, document, photo, or video messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file\_id or specify a URL. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    pub fn edit_message_media(&self, media: impl Into<InputMedia>) -> EditMessageMediaRequest {
        EditMessageMediaRequest::new(self, media)
    }
}

// Divider: all content below this line will be preserved after code regen
