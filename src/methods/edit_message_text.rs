use crate::api::API;
use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::entities::message::Message;
use crate::entities::message_entity::MessageEntity;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct EditMessageTextParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<MessageEntity>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub disable_web_page_preview: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl_into_future!(EditMessageTextRequest<'a>);

///Use this method to edit text and [game](https://core.telegram.org/bots/api/#games) messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
#[derive(Clone)]
pub struct EditMessageTextRequest<'a> {
    api: &'a API,
    params: EditMessageTextParams,
}

impl<'a> RequestT for EditMessageTextRequest<'a> {
    type ParamsType = EditMessageTextParams;
    type ReturnType = Option<Message>;
    fn get_name() -> &'static str {
        "editMessageText"
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
impl<'a> EditMessageTextRequest<'a> {
    pub fn new(api: &'a API, text: String) -> Self {
        Self {
            api,
            params: EditMessageTextParams {
                text,
                chat_id: Option::default(),
                message_id: Option::default(),
                inline_message_id: Option::default(),
                parse_mode: Option::default(),
                entities: Vec::default(),
                disable_web_page_preview: bool::default(),
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

    ///New text of the message, 1-4096 characters after entities parsing
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.params.text = text.into();
        self
    }

    ///Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
        self.params.parse_mode = Some(parse_mode.into());
        self
    }

    ///A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse\_mode*
    pub fn entities(mut self, entities: impl Into<Vec<MessageEntity>>) -> Self {
        self.params.entities = entities.into();
        self
    }

    ///Disables link previews for links in this message
    pub fn disable_web_page_preview(mut self, disable_web_page_preview: impl Into<bool>) -> Self {
        self.params.disable_web_page_preview = disable_web_page_preview.into();
        self
    }

    ///A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl<'a> API {
    ///Use this method to edit text and [game](https://core.telegram.org/bots/api/#games) messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
    pub fn edit_message_text(&'a self, text: impl Into<String>) -> EditMessageTextRequest {
        EditMessageTextRequest::new(self, text.into())
    }
}

// Divider: all content below this line will be preserved after code regen

impl EditMessageTextRequest<'_> {
    pub fn remove_reply_markup(mut self) -> Self {
        self.params.reply_markup = None;
        self
    }
}
