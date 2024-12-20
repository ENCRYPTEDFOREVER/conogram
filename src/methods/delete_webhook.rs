use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API, errors::ConogramError, impl_into_future, request::RequestT,
    utils::deserialize_utils::is_false,
};

#[derive(Debug, Clone, Serialize)]
pub struct DeleteWebhookParams {
    #[serde(default, skip_serializing_if = "is_false")]
    pub drop_pending_updates: bool,
}

impl_into_future!(DeleteWebhookRequest<'a>);

///Use this method to remove webhook integration if you decide to switch back to [getUpdates](https://core.telegram.org/bots/api/#getupdates). Returns *True* on success.
#[derive(Clone)]
pub struct DeleteWebhookRequest<'a> {
    api: &'a API,
    params: DeleteWebhookParams,
}

impl<'a> RequestT for DeleteWebhookRequest<'a> {
    type ParamsType = DeleteWebhookParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "deleteWebhook"
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
impl<'a> DeleteWebhookRequest<'a> {
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: DeleteWebhookParams {
                drop_pending_updates: bool::default(),
            },
        }
    }

    ///Pass *True* to drop all pending updates
    #[must_use]
    pub fn drop_pending_updates(mut self, drop_pending_updates: impl Into<bool>) -> Self {
        self.params.drop_pending_updates = drop_pending_updates.into();
        self
    }
}

impl API {
    ///Use this method to remove webhook integration if you decide to switch back to [getUpdates](https://core.telegram.org/bots/api/#getupdates). Returns *True* on success.
    pub fn delete_webhook(&self) -> DeleteWebhookRequest {
        DeleteWebhookRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
