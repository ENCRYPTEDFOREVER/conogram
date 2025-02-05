


use serde::Serialize;

use crate::{
    api::Api,
    entities::{
        inline_query_result::InlineQueryResult,
        inline_query_results_button::InlineQueryResultsButton,
    },
    
    impl_into_future,
    request::RequestT,
    utils::deserialize_utils::is_false,
};

#[derive(Debug, Clone, Serialize)]

pub struct AnswerInlineQueryParams {
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
    #[serde(default, skip_serializing_if = "is_false")]
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
    api: &'a Api,
    params: AnswerInlineQueryParams,
}

impl RequestT for AnswerInlineQueryRequest<'_> {
    type ParamsType = AnswerInlineQueryParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "answerInlineQuery"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> AnswerInlineQueryRequest<'a> {
    pub fn new(
        api: &'a Api,
        inline_query_id: impl Into<String>,
        results: impl IntoIterator<Item = impl Into<InlineQueryResult>>,
    ) -> Self {
        Self {
            api,
            params: AnswerInlineQueryParams {
                inline_query_id: inline_query_id.into(),
                results: results.into_iter().map(Into::into).collect(),
                cache_time: Option::default(),
                is_personal: bool::default(),
                next_offset: Option::default(),
                button: Option::default(),
            },
        }
    }

    ///Unique identifier for the answered query
    #[must_use]
    pub fn inline_query_id(mut self, inline_query_id: impl Into<String>) -> Self {
        self.params.inline_query_id = inline_query_id.into();
        self
    }

    ///A JSON-serialized array of results for the inline query
    #[must_use]
    pub fn results(
        mut self,
        results: impl IntoIterator<Item = impl Into<InlineQueryResult>>,
    ) -> Self {
        self.params.results = results.into_iter().map(Into::into).collect();
        self
    }

    ///The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    #[must_use]
    pub fn cache_time(mut self, cache_time: impl Into<i64>) -> Self {
        self.params.cache_time = Some(cache_time.into());
        self
    }

    ///Pass *True* if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query.
    #[must_use]
    pub fn is_personal(mut self, is_personal: impl Into<bool>) -> Self {
        self.params.is_personal = is_personal.into();
        self
    }

    ///Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.
    #[must_use]
    pub fn next_offset(mut self, next_offset: impl Into<String>) -> Self {
        self.params.next_offset = Some(next_offset.into());
        self
    }

    ///A JSON-serialized object describing a button to be shown above inline query results
    #[must_use]
    pub fn button(mut self, button: impl Into<InlineQueryResultsButton>) -> Self {
        self.params.button = Some(button.into());
        self
    }
}

impl Api {
    ///Use this method to send answers to an inline query. On success, *True* is returned.  
    ///No more than **50** results per query are allowed.
    pub fn answer_inline_query(
        &self,
        inline_query_id: impl Into<String>,
        results: impl IntoIterator<Item = impl Into<InlineQueryResult>>,
    ) -> AnswerInlineQueryRequest {
        AnswerInlineQueryRequest::new(self, inline_query_id, results)
    }
}

// Divider: all content below this line will be preserved after code regen
