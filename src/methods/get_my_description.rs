use crate::api::API;
use crate::entities::bot_description::BotDescription;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct GetMyDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl_into_future!(GetMyDescriptionRequest<'a>);

///Use this method to get the current bot description for the given user language. Returns [BotDescription](https://core.telegram.org/bots/api/#botdescription) on success.
#[derive(Clone)]
pub struct GetMyDescriptionRequest<'a> {
    api: &'a API,
    params: GetMyDescriptionParams,
}

impl<'a> RequestT for GetMyDescriptionRequest<'a> {
    type ParamsType = GetMyDescriptionParams;
    type ReturnType = BotDescription;
    fn get_name() -> &'static str {
        "getMyDescription"
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
impl<'a> GetMyDescriptionRequest<'a> {
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: GetMyDescriptionParams {
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

impl API {
    ///Use this method to get the current bot description for the given user language. Returns [BotDescription](https://core.telegram.org/bots/api/#botdescription) on success.
    pub fn get_my_description(&self) -> GetMyDescriptionRequest {
        GetMyDescriptionRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
