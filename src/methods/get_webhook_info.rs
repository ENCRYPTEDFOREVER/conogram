use crate::api::API;
use crate::entities::webhook_info::WebhookInfo;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct GetWebhookInfoParams {}

impl_into_future!(GetWebhookInfoRequest<'a>);

///Use this method to get current webhook status. Requires no parameters. On success, returns a [WebhookInfo](https://core.telegram.org/bots/api/#webhookinfo) object. If the bot is using [getUpdates](https://core.telegram.org/bots/api/#getupdates), will return an object with the *url* field empty.
#[derive(Clone)]
pub struct GetWebhookInfoRequest<'a> {
    api: &'a API,
    params: GetWebhookInfoParams,
}

impl<'a> RequestT for GetWebhookInfoRequest<'a> {
    type ParamsType = GetWebhookInfoParams;
    type ReturnType = WebhookInfo;
    fn get_name() -> &'static str {
        "getWebhookInfo"
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
impl<'a> GetWebhookInfoRequest<'a> {
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: GetWebhookInfoParams {},
        }
    }
}

impl<'a> API {
    ///Use this method to get current webhook status. Requires no parameters. On success, returns a [WebhookInfo](https://core.telegram.org/bots/api/#webhookinfo) object. If the bot is using [getUpdates](https://core.telegram.org/bots/api/#getupdates), will return an object with the *url* field empty.
    pub fn get_webhook_info(&'a self) -> GetWebhookInfoRequest {
        GetWebhookInfoRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
