use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{
        inline_query_result::InlineQueryResult,
        inline_query_results_button::InlineQueryResultsButton,
    },
    utils::deserialize_utils::is_false,
};

/// Use this method to send answers to an inline query. On success, *True* is returned.  
/// No more than **50** results per query are allowed.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#answerinlinequery)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct AnswerInlineQueryParams {
    /// Unique identifier for the answered query
    pub inline_query_id: String,

    /// A JSON-serialized array of results for the inline query
    pub results: Vec<InlineQueryResult>,

    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,

    /// Pass *True* if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query.
    #[serde(skip_serializing_if = "is_false")]
    pub is_personal: bool,

    /// Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,

    /// A JSON-serialized object describing a button to be shown above inline query results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<InlineQueryResultsButton>,
}

// Divider: all content below this line will be preserved after code regen
