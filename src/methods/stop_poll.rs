use crate::api::API;
use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::entities::misc::chat_id::ChatId;
use crate::entities::poll::Poll;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct StopPollParams {
    pub chat_id: ChatId,
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl_into_future!(StopPollRequest<'a>);

///Use this method to stop a poll which was sent by the bot. On success, the stopped [Poll](https://core.telegram.org/bots/api/#poll) is returned.
#[derive(Clone)]
pub struct StopPollRequest<'a> {
    api: &'a API,
    params: StopPollParams,
}

impl<'a> RequestT for StopPollRequest<'a> {
    type ParamsType = StopPollParams;
    type ReturnType = Poll;
    fn get_name() -> &'static str {
        "stopPoll"
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
impl<'a> StopPollRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, message_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: StopPollParams {
                chat_id: chat_id.into(),
                message_id: message_id.into(),
                reply_markup: Option::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Identifier of the original message with the poll
    pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
        self.params.message_id = message_id.into();
        self
    }

    ///A JSON-serialized object for a new message [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl<'a> API {
    ///Use this method to stop a poll which was sent by the bot. On success, the stopped [Poll](https://core.telegram.org/bots/api/#poll) is returned.
    pub fn stop_poll(
        &'a self,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<i64>,
    ) -> StopPollRequest {
        StopPollRequest::new(self, chat_id, message_id)
    }
}

// Divider: all content below this line will be preserved after code regen
