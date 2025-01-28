use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{api::Api, errors::ConogramError, impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]
pub struct CloseParams {}

impl_into_future!(CloseRequest<'a>);

///Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns *True* on success. Requires no parameters.
#[derive(Clone)]
pub struct CloseRequest<'a> {
    api: &'a Api,
    params: CloseParams,
}

impl RequestT for CloseRequest<'_> {
    type ParamsType = CloseParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "close"
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
impl<'a> CloseRequest<'a> {
    pub const fn new(api: &'a Api) -> Self {
        Self {
            api,
            params: CloseParams {},
        }
    }
}

impl Api {
    ///Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns *True* on success. Requires no parameters.
    pub const fn close(&self) -> CloseRequest {
        CloseRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
