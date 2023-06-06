use crate::api::API;
use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::entities::message::Message;
use crate::entities::message_entity::MessageEntity;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct EditMessageCaptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl_into_future!(EditMessageCaptionRequest<'a>);

///Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
#[derive(Clone)]
pub struct EditMessageCaptionRequest<'a> {
    api: &'a API,
    params: EditMessageCaptionParams,
}

impl<'a> RequestT for EditMessageCaptionRequest<'a> {
    type ParamsType = EditMessageCaptionParams;
    type ReturnType = Option<Message>;
    fn get_name() -> &'static str {
        "editMessageCaption"
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
impl<'a> EditMessageCaptionRequest<'a> {
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: EditMessageCaptionParams {
                chat_id: Option::default(),
                message_id: Option::default(),
                inline_message_id: Option::default(),
                caption: Option::default(),
                parse_mode: Option::default(),
                caption_entities: Vec::default(),
                reply_markup: Option::default(),
            },
        }
    }

    ///Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = Some(chat_id.into());
        self
    }

    ///Required if *inline\_message\_id* is not specified. Identifier of the message to edit
    pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
        self.params.message_id = Some(message_id.into());
        self
    }

    ///Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message
    pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(inline_message_id.into());
        self
    }

    ///New caption of the message, 0-1024 characters after entities parsing
    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.params.caption = Some(caption.into());
        self
    }

    ///Mode for parsing entities in the message caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
        self.params.parse_mode = Some(parse_mode.into());
        self
    }

    ///A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
        self.params.caption_entities = caption_entities.into();
        self
    }

    ///A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl<'a> API {
    ///Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
    pub fn edit_message_caption(&'a self) -> EditMessageCaptionRequest {
        EditMessageCaptionRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
