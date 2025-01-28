use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api, entities::business_connection::BusinessConnection, errors::ConogramError,
    impl_into_future, request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetBusinessConnectionParams {
    pub business_connection_id: String,
}

impl_into_future!(GetBusinessConnectionRequest<'a>);

///Use this method to get information about the connection of the bot with a business account. Returns a [BusinessConnection](https://core.telegram.org/bots/api/#businessconnection) object on success.
#[derive(Clone)]
pub struct GetBusinessConnectionRequest<'a> {
    api: &'a Api,
    params: GetBusinessConnectionParams,
}

impl RequestT for GetBusinessConnectionRequest<'_> {
    type ParamsType = GetBusinessConnectionParams;
    type ReturnType = BusinessConnection;
    fn get_name() -> &'static str {
        "getBusinessConnection"
    }
    fn get_api_ref(&self) -> &Api {
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
    pub fn new(api: &'a Api, business_connection_id: impl Into<String>) -> Self {
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

impl Api {
    ///Use this method to get information about the connection of the bot with a business account. Returns a [BusinessConnection](https://core.telegram.org/bots/api/#businessconnection) object on success.
    pub fn get_business_connection(
        &self,
        business_connection_id: impl Into<String>,
    ) -> GetBusinessConnectionRequest {
        GetBusinessConnectionRequest::new(self, business_connection_id)
    }
}

// Divider: all content below this line will be preserved after code regen
