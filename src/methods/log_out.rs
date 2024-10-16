use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{api::API, errors::ConogramError, impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]
pub struct LogOutParams {}

impl_into_future!(LogOutRequest<'a>);

///Use this method to log out from the cloud Bot API server before launching the bot locally. You **must** log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns *True* on success. Requires no parameters.
#[derive(Clone)]
pub struct LogOutRequest<'a> {
    api: &'a API,
    params: LogOutParams,
}

impl<'a> RequestT for LogOutRequest<'a> {
    type ParamsType = LogOutParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "logOut"
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
impl<'a> LogOutRequest<'a> {
    pub const fn new(api: &'a API) -> Self {
        Self {
            api,
            params: LogOutParams {},
        }
    }
}

impl API {
    ///Use this method to log out from the cloud Bot API server before launching the bot locally. You **must** log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns *True* on success. Requires no parameters.
    pub const fn log_out(&self) -> LogOutRequest {
        LogOutRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
