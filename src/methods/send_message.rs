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
pub struct SendMessageParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<MessageEntity>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub disable_web_page_preview: bool,
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

impl_into_future!(SendMessageRequest<'a>);

///Use this method to send text messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
#[derive(Clone)]
pub struct SendMessageRequest<'a> {
    api: &'a API,
    params: SendMessageParams,
}

impl<'a> RequestT for SendMessageRequest<'a> {
    type ParamsType = SendMessageParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "sendMessage"
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
impl<'a> SendMessageRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId, text: String) -> Self {
        Self {
            api,
            params: SendMessageParams {
                chat_id,
                text,
                message_thread_id: Option::default(),
                parse_mode: Option::default(),
                entities: Vec::default(),
                disable_web_page_preview: bool::default(),
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

    ///Text of the message to be sent, 1-4096 characters after entities parsing
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
    ///Use this method to send text messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn send_message(
        &'a self,
        chat_id: impl Into<ChatId>,
        text: impl Into<String>,
    ) -> SendMessageRequest {
        SendMessageRequest::new(self, chat_id.into(), text.into())
    }
}

// Divider: all content below this line will be preserved after code regen
