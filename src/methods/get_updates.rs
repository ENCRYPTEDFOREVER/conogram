use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API, entities::update::Update, errors::ConogramError, impl_into_future, request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetUpdatesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_updates: Vec<String>,
}

impl_into_future!(GetUpdatesRequest<'a>);

///Use this method to receive incoming updates using long polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)). Returns an Array of [Update](https://core.telegram.org/bots/api/#update) objects.
#[derive(Clone)]
pub struct GetUpdatesRequest<'a> {
    api: &'a API,
    params: GetUpdatesParams,
}

impl<'a> RequestT for GetUpdatesRequest<'a> {
    type ParamsType = GetUpdatesParams;
    type ReturnType = Vec<Update>;
    fn get_name() -> &'static str {
        "getUpdates"
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
impl<'a> GetUpdatesRequest<'a> {
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: GetUpdatesParams {
                offset: Option::default(),
                limit: Option::default(),
                timeout: Option::default(),
                allowed_updates: Vec::default(),
            },
        }
    }

    ///Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as [getUpdates](https://core.telegram.org/bots/api/#getupdates) is called with an *offset* higher than its *update\_id*. The negative offset can be specified to retrieve updates starting from *-offset* update from the end of the updates queue. All previous updates will be forgotten.
    #[must_use]
    pub fn offset(mut self, offset: impl Into<i64>) -> Self {
        self.params.offset = Some(offset.into());
        self
    }

    ///Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[must_use]
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.params.limit = Some(limit.into());
        self
    }

    ///Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    #[must_use]
    pub fn timeout(mut self, timeout: impl Into<i64>) -> Self {
        self.params.timeout = Some(timeout.into());
        self
    }

    ///A JSON-serialized list of the update types you want your bot to receive. For example, specify `["message", "edited_channel_post", "callback_query"]` to only receive updates of these types. See [Update](https://core.telegram.org/bots/api/#update) for a complete list of available update types. Specify an empty list to receive all update types except *chat\_member*, *message\_reaction*, and *message\_reaction\_count* (default). If not specified, the previous setting will be used.  
    ///
    ///Please note that this parameter doesn't affect updates created before the call to the getUpdates, so unwanted updates may be received for a short period of time.
    #[must_use]
    pub fn allowed_updates(
        mut self,
        allowed_updates: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.params.allowed_updates = allowed_updates.into_iter().map(Into::into).collect();
        self
    }
}

impl API {
    ///Use this method to receive incoming updates using long polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)). Returns an Array of [Update](https://core.telegram.org/bots/api/#update) objects.
    pub fn get_updates(&self) -> GetUpdatesRequest {
        GetUpdatesRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
