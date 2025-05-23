use serde::{Deserialize, Serialize};

use crate::entities::{location::Location, user::User};

/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inlinequery)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,

    /// Sender
    pub from: User,

    /// Text of the query (up to 256 characters)
    pub query: String,

    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,

    /// *Optional*. Type of the chat from which the inline query was sent. Can be either “sender” for a private chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<ChatType>,

    /// *Optional*. Sender location, only for bots that request user location
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

/// *Optional*. Type of the chat from which the inline query was sent. Can be either “sender” for a private chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatType {
    /// `sender`
    #[default]
    #[serde(rename = "sender")]
    Sender,

    /// `private`
    #[serde(rename = "private")]
    Private,

    /// `group`
    #[serde(rename = "group")]
    Group,

    /// `supergroup`
    #[serde(rename = "supergroup")]
    Supergroup,

    /// `channel`
    #[serde(rename = "channel")]
    Channel,
}

// Divider: all content below this line will be preserved after code regen
use super::inline_query_result::InlineQueryResult;
use crate::{api::Api, methods::answer_inline_query::AnswerInlineQueryRequest};

impl InlineQuery {
    pub fn answer<'a>(
        &'a self,
        api: &'a Api,
        results: impl IntoIterator<Item = impl Into<InlineQueryResult>>,
    ) -> AnswerInlineQueryRequest<'a> {
        api.answer_inline_query(&self.id, results)
    }

    // Answer with no results
    pub fn answer_empty<'a>(&'a self, api: &'a Api) -> AnswerInlineQueryRequest<'a> {
        api.answer_inline_query(&self.id, Vec::<InlineQueryResult>::new())
    }

    /// Answer with all server-side caching disabled
    pub fn answer_persnocache<'a, T: Into<InlineQueryResult>>(
        &'a self,
        api: &'a Api,
        results: impl IntoIterator<Item = T>,
    ) -> AnswerInlineQueryRequest<'a> {
        self.answer(api, results).is_personal(true).cache_time(0)
    }
}
