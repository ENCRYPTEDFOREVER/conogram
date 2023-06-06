use crate::entities::location::Location;
use crate::entities::user::User;
use serde::{Deserialize, Serialize};

///This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
///API Reference: [link](https://core.telegram.org/bots/api/#inlinequery)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct InlineQuery {
    ///Unique identifier for this query
    pub id: String,

    ///Sender
    pub from: User,

    ///Text of the query (up to 256 characters)
    pub query: String,

    ///Offset of the results to be returned, can be controlled by the bot
    pub offset: String,

    ///*Optional*. Type of the chat from which the inline query was sent. Can be either “sender” for a private chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<InlineQueryChatType>,

    ///*Optional*. Sender location, only for bots that request user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

///*Optional*. Type of the chat from which the inline query was sent. Can be either “sender” for a private chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "chat_type")]
pub enum InlineQueryChatType {
    #[default]
    /// "sender"
    #[serde(rename = "sender")]
    Sender,

    /// "private"
    #[serde(rename = "private")]
    Private,

    /// "group"
    #[serde(rename = "group")]
    Group,

    /// "supergroup"
    #[serde(rename = "supergroup")]
    Supergroup,

    /// "channel"
    #[serde(rename = "channel")]
    Channel,
}
// Divider: all content below this line will be preserved after code regen
use super::inline_query_result::InlineQueryResult;
use crate::api::API;
use crate::methods::answer_inline_query::AnswerInlineQueryRequest;

impl InlineQuery {
    pub fn answer<'a>(
        &'a self,
        api: &'a API,
        results: impl Into<Vec<InlineQueryResult>>,
    ) -> AnswerInlineQueryRequest {
        api.answer_inline_query(&self.id, results.into())
    }

    pub fn answer_persnocache<'a>(
        &'a self,
        api: &'a API,
        results: impl Into<Vec<InlineQueryResult>>,
    ) -> AnswerInlineQueryRequest {
        self.answer(api, results).is_personal(true).cache_time(0)
    }
}
