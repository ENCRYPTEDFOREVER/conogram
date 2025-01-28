use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api, entities::user::User, errors::ConogramError, impl_into_future, request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetMeParams {}

impl_into_future!(GetMeRequest<'a>);

///A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a [User](https://core.telegram.org/bots/api/#user) object.
#[derive(Clone)]
pub struct GetMeRequest<'a> {
    api: &'a Api,
    params: GetMeParams,
}

impl RequestT for GetMeRequest<'_> {
    type ParamsType = GetMeParams;
    type ReturnType = User;
    fn get_name() -> &'static str {
        "getMe"
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
impl<'a> GetMeRequest<'a> {
    pub const fn new(api: &'a Api) -> Self {
        Self {
            api,
            params: GetMeParams {},
        }
    }
}

impl Api {
    ///A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a [User](https://core.telegram.org/bots/api/#user) object.
    pub const fn get_me(&self) -> GetMeRequest {
        GetMeRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
