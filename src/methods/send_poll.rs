use crate::api::API;
use crate::entities::message::Message;
use crate::entities::message_entity::MessageEntity;
use crate::entities::misc::chat_id::ChatId;
use crate::entities::misc::reply_markup::ReplyMarkup;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SendPollParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub question: String,
    pub options: Vec<String>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_anonymous: bool,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<SendPollType>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub allows_multiple_answers: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub explanation_entities: Vec<MessageEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_closed: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub disable_notification: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub protect_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub allow_sending_without_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl_into_future!(SendPollRequest<'a>);

///Use this method to send a native poll. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
#[derive(Clone)]
pub struct SendPollRequest<'a> {
    api: &'a API,
    params: SendPollParams,
}

impl<'a> RequestT for SendPollRequest<'a> {
    type ParamsType = SendPollParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "sendPoll"
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
impl<'a> SendPollRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId, question: String, options: Vec<String>) -> Self {
        Self {
            api,
            params: SendPollParams {
                chat_id,
                question,
                options,
                message_thread_id: Option::default(),
                is_anonymous: bool::default(),
                type_: Option::default(),
                allows_multiple_answers: bool::default(),
                correct_option_id: Option::default(),
                explanation: Option::default(),
                explanation_parse_mode: Option::default(),
                explanation_entities: Vec::default(),
                open_period: Option::default(),
                close_date: Option::default(),
                is_closed: bool::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
                reply_to_message_id: Option::default(),
                allow_sending_without_reply: bool::default(),
                reply_markup: Option::default(),
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

    ///Poll question, 1-300 characters
    pub fn question(mut self, question: impl Into<String>) -> Self {
        self.params.question = question.into();
        self
    }

    ///A JSON-serialized list of answer options, 2-10 strings 1-100 characters each
    pub fn options(mut self, options: impl Into<Vec<String>>) -> Self {
        self.params.options = options.into();
        self
    }

    ///*True*, if the poll needs to be anonymous, defaults to *True*
    pub fn is_anonymous(mut self, is_anonymous: impl Into<bool>) -> Self {
        self.params.is_anonymous = is_anonymous.into();
        self
    }

    pub fn type_(mut self, type_: impl Into<SendPollType>) -> Self {
        self.params.type_ = Some(type_.into());
        self
    }

    ///*True*, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to *False*
    pub fn allows_multiple_answers(mut self, allows_multiple_answers: impl Into<bool>) -> Self {
        self.params.allows_multiple_answers = allows_multiple_answers.into();
        self
    }

    ///0-based identifier of the correct answer option, required for polls in quiz mode
    pub fn correct_option_id(mut self, correct_option_id: impl Into<i64>) -> Self {
        self.params.correct_option_id = Some(correct_option_id.into());
        self
    }

    ///Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    pub fn explanation(mut self, explanation: impl Into<String>) -> Self {
        self.params.explanation = Some(explanation.into());
        self
    }

    ///Mode for parsing entities in the explanation. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    pub fn explanation_parse_mode(mut self, explanation_parse_mode: impl Into<String>) -> Self {
        self.params.explanation_parse_mode = Some(explanation_parse_mode.into());
        self
    }

    ///A JSON-serialized list of special entities that appear in the poll explanation, which can be specified instead of *parse\_mode*
    pub fn explanation_entities(
        mut self,
        explanation_entities: impl Into<Vec<MessageEntity>>,
    ) -> Self {
        self.params.explanation_entities = explanation_entities.into();
        self
    }

    ///Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with *close\_date*.
    pub fn open_period(mut self, open_period: impl Into<i64>) -> Self {
        self.params.open_period = Some(open_period.into());
        self
    }

    ///Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with *open\_period*.
    pub fn close_date(mut self, close_date: impl Into<i64>) -> Self {
        self.params.close_date = Some(close_date.into());
        self
    }

    ///Pass *True* if the poll needs to be immediately closed. This can be useful for poll preview.
    pub fn is_closed(mut self, is_closed: impl Into<bool>) -> Self {
        self.params.is_closed = is_closed.into();
        self
    }

    ///Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: impl Into<bool>) -> Self {
        self.params.protect_content = protect_content.into();
        self
    }

    ///If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: impl Into<i64>) -> Self {
        self.params.reply_to_message_id = Some(reply_to_message_id.into());
        self
    }

    ///Pass *True* if the message should be sent even if the specified replied-to message is not found
    pub fn allow_sending_without_reply(
        mut self,
        allow_sending_without_reply: impl Into<bool>,
    ) -> Self {
        self.params.allow_sending_without_reply = allow_sending_without_reply.into();
        self
    }

    ///Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl<'a> API {
    ///Use this method to send a native poll. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn send_poll(
        &'a self,
        chat_id: impl Into<ChatId>,
        question: impl Into<String>,
        options: impl Into<Vec<String>>,
    ) -> SendPollRequest {
        SendPollRequest::new(self, chat_id.into(), question.into(), options.into())
    }
}

///Poll type, “quiz” or “regular”, defaults to “regular”
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "type")]
pub enum SendPollType {
    #[default]
    /// "quiz"
    #[serde(rename = "quiz")]
    Quiz,

    /// "regular"
    #[serde(rename = "regular")]
    Regular,
}
// Divider: all content below this line will be preserved after code regen
