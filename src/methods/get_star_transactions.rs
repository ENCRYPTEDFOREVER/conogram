use crate::api::API;
use crate::entities::star_transactions::StarTransactions;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct GetStarTransactionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl_into_future!(GetStarTransactionsRequest<'a>);

///Returns the bot's Telegram Star transactions in chronological order. On success, returns a [StarTransactions](https://core.telegram.org/bots/api/#startransactions) object.
#[derive(Clone)]
pub struct GetStarTransactionsRequest<'a> {
    api: &'a API,
    params: GetStarTransactionsParams,
}

impl<'a> RequestT for GetStarTransactionsRequest<'a> {
    type ParamsType = GetStarTransactionsParams;
    type ReturnType = StarTransactions;
    fn get_name() -> &'static str {
        "getStarTransactions"
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
impl<'a> GetStarTransactionsRequest<'a> {
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: GetStarTransactionsParams {
                offset: Option::default(),
                limit: Option::default(),
            },
        }
    }

    ///Number of transactions to skip in the response
    #[must_use]
    pub fn offset(mut self, offset: impl Into<i64>) -> Self {
        self.params.offset = Some(offset.into());
        self
    }

    ///The maximum number of transactions to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[must_use]
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.params.limit = Some(limit.into());
        self
    }
}

impl<'a> API {
    ///Returns the bot's Telegram Star transactions in chronological order. On success, returns a [StarTransactions](https://core.telegram.org/bots/api/#startransactions) object.
    pub fn get_star_transactions(&'a self) -> GetStarTransactionsRequest {
        GetStarTransactionsRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
