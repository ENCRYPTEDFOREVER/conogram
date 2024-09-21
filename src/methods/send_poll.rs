use crate::api::API;
use crate::entities::input_poll_option::InputPollOption;
use crate::entities::message::Message;
use crate::entities::message_entity::MessageEntity;
use crate::entities::misc::chat_id::ChatId;
use crate::entities::misc::reply_markup::ReplyMarkup;
use crate::entities::reply_parameters::ReplyParameters;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SendPollParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub question_entities: Vec<MessageEntity>,
    pub options: Vec<InputPollOption>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_anonymous: bool,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<SendPollType>,
    #[serde(default, skip_serializing_if = "is_false")]
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
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_closed: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_notification: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub protect_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
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
    pub fn new(
        api: &'a API,
        chat_id: impl Into<ChatId>,
        question: impl Into<String>,
        options: impl IntoIterator<Item = impl Into<InputPollOption>>,
    ) -> Self {
        Self {
            api,
            params: SendPollParams {
                chat_id: chat_id.into(),
                question: question.into(),
                options: options.into_iter().map(Into::into).collect(),
                business_connection_id: Option::default(),
                message_thread_id: Option::default(),
                question_parse_mode: Option::default(),
                question_entities: Vec::default(),
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
                message_effect_id: Option::default(),
                reply_parameters: Option::default(),
                reply_markup: Option::default(),
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

    ///Poll question, 1-300 characters
    #[must_use]
    pub fn question(mut self, question: impl Into<String>) -> Self {
        self.params.question = question.into();
        self
    }

    ///Mode for parsing entities in the question. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Currently, only custom emoji entities are allowed
    #[must_use]
    pub fn question_parse_mode(mut self, question_parse_mode: impl Into<String>) -> Self {
        self.params.question_parse_mode = Some(question_parse_mode.into());
        self
    }

    ///A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of *question\_parse\_mode*
    #[must_use]
    pub fn question_entities(
        mut self,
        question_entities: impl IntoIterator<Item = impl Into<MessageEntity>>,
    ) -> Self {
        self.params.question_entities = question_entities.into_iter().map(Into::into).collect();
        self
    }

    ///A JSON-serialized list of 2-10 answer options
    #[must_use]
    pub fn options(
        mut self,
        options: impl IntoIterator<Item = impl Into<InputPollOption>>,
    ) -> Self {
        self.params.options = options.into_iter().map(Into::into).collect();
        self
    }

    ///*True*, if the poll needs to be anonymous, defaults to *True*
    #[must_use]
    pub fn is_anonymous(mut self, is_anonymous: impl Into<bool>) -> Self {
        self.params.is_anonymous = is_anonymous.into();
        self
    }

    #[must_use]
    pub fn type_(mut self, type_: impl Into<SendPollType>) -> Self {
        self.params.type_ = Some(type_.into());
        self
    }

    ///*True*, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to *False*
    #[must_use]
    pub fn allows_multiple_answers(mut self, allows_multiple_answers: impl Into<bool>) -> Self {
        self.params.allows_multiple_answers = allows_multiple_answers.into();
        self
    }

    ///0-based identifier of the correct answer option, required for polls in quiz mode
    #[must_use]
    pub fn correct_option_id(mut self, correct_option_id: impl Into<i64>) -> Self {
        self.params.correct_option_id = Some(correct_option_id.into());
        self
    }

    ///Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[must_use]
    pub fn explanation(mut self, explanation: impl Into<String>) -> Self {
        self.params.explanation = Some(explanation.into());
        self
    }

    ///Mode for parsing entities in the explanation. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[must_use]
    pub fn explanation_parse_mode(mut self, explanation_parse_mode: impl Into<String>) -> Self {
        self.params.explanation_parse_mode = Some(explanation_parse_mode.into());
        self
    }

    ///A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of *explanation\_parse\_mode*
    #[must_use]
    pub fn explanation_entities(
        mut self,
        explanation_entities: impl IntoIterator<Item = impl Into<MessageEntity>>,
    ) -> Self {
        self.params.explanation_entities =
            explanation_entities.into_iter().map(Into::into).collect();
        self
    }

    ///Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with *close\_date*.
    #[must_use]
    pub fn open_period(mut self, open_period: impl Into<i64>) -> Self {
        self.params.open_period = Some(open_period.into());
        self
    }

    ///Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with *open\_period*.
    #[must_use]
    pub fn close_date(mut self, close_date: impl Into<i64>) -> Self {
        self.params.close_date = Some(close_date.into());
        self
    }

    ///Pass *True* if the poll needs to be immediately closed. This can be useful for poll preview.
    #[must_use]
    pub fn is_closed(mut self, is_closed: impl Into<bool>) -> Self {
        self.params.is_closed = is_closed.into();
        self
    }

    ///Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content(mut self, protect_content: impl Into<bool>) -> Self {
        self.params.protect_content = protect_content.into();
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

    ///Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
    #[must_use]
    pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl API {
    ///Use this method to send a native poll. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn send_poll(
        &self,
        chat_id: impl Into<ChatId>,
        question: impl Into<String>,
        options: impl IntoIterator<Item = impl Into<InputPollOption>>,
    ) -> SendPollRequest {
        SendPollRequest::new(self, chat_id, question, options)
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
