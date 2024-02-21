use crate::api::API;
use crate::entities::inline_query_result::InlineQueryResult;
use crate::entities::sent_web_app_message::SentWebAppMessage;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct AnswerWebAppQueryParams {
    pub web_app_query_id: String,
    pub result: InlineQueryResult,
}

impl_into_future!(AnswerWebAppQueryRequest<'a>);

///Use this method to set the result of an interaction with a [Web App](https://core.telegram.org/bots/webapps) and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a [SentWebAppMessage](https://core.telegram.org/bots/api/#sentwebappmessage) object is returned.
#[derive(Clone)]
pub struct AnswerWebAppQueryRequest<'a> {
    api: &'a API,
    params: AnswerWebAppQueryParams,
}

impl<'a> RequestT for AnswerWebAppQueryRequest<'a> {
    type ParamsType = AnswerWebAppQueryParams;
    type ReturnType = SentWebAppMessage;
    fn get_name() -> &'static str {
        "answerWebAppQuery"
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
impl<'a> AnswerWebAppQueryRequest<'a> {
    pub fn new(
        api: &'a API,
        web_app_query_id: impl Into<String>,
        result: impl Into<InlineQueryResult>,
    ) -> Self {
        Self {
            api,
            params: AnswerWebAppQueryParams {
                web_app_query_id: web_app_query_id.into(),
                result: result.into(),
            },
        }
    }

    ///Unique identifier for the query to be answered
    pub fn web_app_query_id(mut self, web_app_query_id: impl Into<String>) -> Self {
        self.params.web_app_query_id = web_app_query_id.into();
        self
    }

    ///A JSON-serialized object describing the message to be sent
    pub fn result(mut self, result: impl Into<InlineQueryResult>) -> Self {
        self.params.result = result.into();
        self
    }
}

impl<'a> API {
    ///Use this method to set the result of an interaction with a [Web App](https://core.telegram.org/bots/webapps) and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a [SentWebAppMessage](https://core.telegram.org/bots/api/#sentwebappmessage) object is returned.
    pub fn answer_web_app_query(
        &'a self,
        web_app_query_id: impl Into<String>,
        result: impl Into<InlineQueryResult>,
    ) -> AnswerWebAppQueryRequest {
        AnswerWebAppQueryRequest::new(self, web_app_query_id, result)
    }
}

// Divider: all content below this line will be preserved after code regen
