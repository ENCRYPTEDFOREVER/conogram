use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api, entities::gifts::Gifts, errors::ConogramError, impl_into_future, request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetAvailableGiftsParams {}

impl_into_future!(GetAvailableGiftsRequest<'a>);

///Returns the list of gifts that can be sent by the bot to users. Requires no parameters. Returns a [Gifts](https://core.telegram.org/bots/api/#gifts) object.
#[derive(Clone)]
pub struct GetAvailableGiftsRequest<'a> {
    api: &'a Api,
    params: GetAvailableGiftsParams,
}

impl RequestT for GetAvailableGiftsRequest<'_> {
    type ParamsType = GetAvailableGiftsParams;
    type ReturnType = Gifts;
    fn get_name() -> &'static str {
        "getAvailableGifts"
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
impl<'a> GetAvailableGiftsRequest<'a> {
    pub const fn new(api: &'a Api) -> Self {
        Self {
            api,
            params: GetAvailableGiftsParams {},
        }
    }
}

impl Api {
    ///Returns the list of gifts that can be sent by the bot to users. Requires no parameters. Returns a [Gifts](https://core.telegram.org/bots/api/#gifts) object.
    pub const fn get_available_gifts(&self) -> GetAvailableGiftsRequest {
        GetAvailableGiftsRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
