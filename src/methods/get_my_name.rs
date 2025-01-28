use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api, entities::bot_name::BotName, errors::ConogramError, impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetMyNameParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl_into_future!(GetMyNameRequest<'a>);

///Use this method to get the current bot name for the given user language. Returns [BotName](https://core.telegram.org/bots/api/#botname) on success.
#[derive(Clone)]
pub struct GetMyNameRequest<'a> {
    api: &'a Api,
    params: GetMyNameParams,
}

impl RequestT for GetMyNameRequest<'_> {
    type ParamsType = GetMyNameParams;
    type ReturnType = BotName;
    fn get_name() -> &'static str {
        "getMyName"
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
impl<'a> GetMyNameRequest<'a> {
    pub fn new(api: &'a Api) -> Self {
        Self {
            api,
            params: GetMyNameParams {
                language_code: Option::default(),
            },
        }
    }

    ///A two-letter ISO 639-1 language code or an empty string
    #[must_use]
    pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
        self.params.language_code = Some(language_code.into());
        self
    }
}

impl Api {
    ///Use this method to get the current bot name for the given user language. Returns [BotName](https://core.telegram.org/bots/api/#botname) on success.
    pub fn get_my_name(&self) -> GetMyNameRequest {
        GetMyNameRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
