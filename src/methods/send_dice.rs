use crate::api::API;
use crate::entities::message::Message;
use crate::entities::misc::chat_id::ChatId;
use crate::entities::misc::reply_markup::ReplyMarkup;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SendDiceParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<SendDiceEmoji>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_notification: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub protect_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_sending_without_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl_into_future!(SendDiceRequest<'a>);

///Use this method to send an animated emoji that will display a random value. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
#[derive(Clone)]
pub struct SendDiceRequest<'a> {
    api: &'a API,
    params: SendDiceParams,
}

impl<'a> RequestT for SendDiceRequest<'a> {
    type ParamsType = SendDiceParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "sendDice"
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
impl<'a> SendDiceRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId) -> Self {
        Self {
            api,
            params: SendDiceParams {
                chat_id,
                message_thread_id: Option::default(),
                emoji: Option::default(),
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

    ///Emoji on which the dice throw animation is based. Currently, must be one of “🎲”, “🎯”, “🏀”, “⚽”, “🎳”, or “🎰”. Dice can have values 1-6 for “🎲”, “🎯” and “🎳”, values 1-5 for “🏀” and “⚽”, and values 1-64 for “🎰”. Defaults to “🎲”
    pub fn emoji(mut self, emoji: impl Into<SendDiceEmoji>) -> Self {
        self.params.emoji = Some(emoji.into());
        self
    }

    ///Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the sent message from forwarding
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
    ///Use this method to send an animated emoji that will display a random value. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn send_dice(&'a self, chat_id: impl Into<ChatId>) -> SendDiceRequest {
        SendDiceRequest::new(self, chat_id.into())
    }
}

///Emoji on which the dice throw animation is based. Currently, must be one of “🎲”, “🎯”, “🏀”, “⚽”, “🎳”, or “🎰”. Dice can have values 1-6 for “🎲”, “🎯” and “🎳”, values 1-5 for “🏀” and “⚽”, and values 1-64 for “🎰”. Defaults to “🎲”
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "emoji")]
pub enum SendDiceEmoji {
    #[default]
    /// "🎲"
    #[serde(rename = "🎲")]
    Dice,

    /// "🎯"
    #[serde(rename = "🎯")]
    Dart,

    /// "🏀"
    #[serde(rename = "🏀")]
    BasketBall,

    /// "⚽"
    #[serde(rename = "⚽")]
    FootBall,

    /// "🎳"
    #[serde(rename = "🎳")]
    Bowling,

    /// "🎰"
    #[serde(rename = "🎰")]
    SlotMachine,
}

// Divider: all content below this line will be preserved after code regen
