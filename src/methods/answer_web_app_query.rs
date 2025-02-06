use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{
    inline_query_result::InlineQueryResult, sent_web_app_message::SentWebAppMessage,
};

/// Use this method to set the result of an interaction with a [Web App](https://core.telegram.org/bots/webapps) and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a [SentWebAppMessage](https://core.telegram.org/bots/api/#sentwebappmessage) object is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#answerwebappquery)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = SentWebAppMessage)]
pub struct AnswerWebAppQueryParams {
    /// Unique identifier for the query to be answered
    pub web_app_query_id: String,

    /// A JSON-serialized object describing the message to be sent
    pub result: InlineQueryResult,
}

// Divider: all content below this line will be preserved after code regen
