use crate::api::API;
use crate::entities::inline_query_result::InlineQueryResult;
use crate::entities::inline_query_results_button::InlineQueryResultsButton;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct AnswerInlineQueryParams {
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_personal: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<InlineQueryResultsButton>,
}

impl_into_future!(AnswerInlineQueryRequest<'a>);

///Use this method to send answers to an inline query. On success, *True* is returned.  
///No more than **50** results per query are allowed.
#[derive(Clone)]
pub struct AnswerInlineQueryRequest<'a> {
    api: &'a API,
    params: AnswerInlineQueryParams,
}

impl<'a> RequestT for AnswerInlineQueryRequest<'a> {
    type ParamsType = AnswerInlineQueryParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "answerInlineQuery"
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
impl<'a> AnswerInlineQueryRequest<'a> {
    pub fn new(api: &'a API, inline_query_id: String, results: Vec<InlineQueryResult>) -> Self {
        Self {
            api,
            params: AnswerInlineQueryParams {
                inline_query_id,
                results,
                cache_time: Option::default(),
                is_personal: bool::default(),
                next_offset: Option::default(),
                button: Option::default(),
            },
        }
    }

    ///Unique identifier for the answered query
    pub fn inline_query_id(mut self, inline_query_id: impl Into<String>) -> Self {
        self.params.inline_query_id = inline_query_id.into();
        self
    }

    ///A JSON-serialized array of results for the inline query
    pub fn results(mut self, results: impl Into<Vec<InlineQueryResult>>) -> Self {
        self.params.results = results.into();
        self
    }

    ///The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    pub fn cache_time(mut self, cache_time: impl Into<i64>) -> Self {
        self.params.cache_time = Some(cache_time.into());
        self
    }

    ///Pass *True* if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query.
    pub fn is_personal(mut self, is_personal: impl Into<bool>) -> Self {
        self.params.is_personal = is_personal.into();
        self
    }

    ///Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.
    pub fn next_offset(mut self, next_offset: impl Into<String>) -> Self {
        self.params.next_offset = Some(next_offset.into());
        self
    }

    ///A JSON-serialized object describing a button to be shown above inline query results
    pub fn button(mut self, button: impl Into<InlineQueryResultsButton>) -> Self {
        self.params.button = Some(button.into());
        self
    }
}

impl<'a> API {
    ///Use this method to send answers to an inline query. On success, *True* is returned.  
    ///No more than **50** results per query are allowed.
    pub fn answer_inline_query(
        &'a self,
        inline_query_id: impl Into<String>,
        results: impl Into<Vec<InlineQueryResult>>,
    ) -> AnswerInlineQueryRequest {
        AnswerInlineQueryRequest::new(self, inline_query_id.into(), results.into())
    }
}

// Divider: all content below this line will be preserved after code regen
