use crate::api::API;
use crate::entities::business_connection::BusinessConnection;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct GetBusinessConnectionParams {
    pub business_connection_id: String,
}

impl_into_future!(GetBusinessConnectionRequest<'a>);

///Use this method to get information about the connection of the bot with a business account. Returns a [BusinessConnection](https://core.telegram.org/bots/api/#businessconnection) object on success.
#[derive(Clone)]
pub struct GetBusinessConnectionRequest<'a> {
    api: &'a API,
    params: GetBusinessConnectionParams,
}

impl<'a> RequestT for GetBusinessConnectionRequest<'a> {
    type ParamsType = GetBusinessConnectionParams;
    type ReturnType = BusinessConnection;
    fn get_name() -> &'static str {
        "getBusinessConnection"
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
impl<'a> GetBusinessConnectionRequest<'a> {
    pub fn new(api: &'a API, business_connection_id: impl Into<String>) -> Self {
        Self {
            api,
            params: GetBusinessConnectionParams {
                business_connection_id: business_connection_id.into(),
            },
        }
    }

    ///Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
        self.params.business_connection_id = business_connection_id.into();
        self
    }
}

impl API {
    ///Use this method to get information about the connection of the bot with a business account. Returns a [BusinessConnection](https://core.telegram.org/bots/api/#businessconnection) object on success.
    pub fn get_business_connection(
        &self,
        business_connection_id: impl Into<String>,
    ) -> GetBusinessConnectionRequest {
        GetBusinessConnectionRequest::new(self, business_connection_id)
    }
}

// Divider: all content below this line will be preserved after code regen
