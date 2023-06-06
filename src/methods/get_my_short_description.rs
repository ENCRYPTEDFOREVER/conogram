use crate::api::API;
use crate::entities::bot_short_description::BotShortDescription;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct GetMyShortDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl_into_future!(GetMyShortDescriptionRequest<'a>);

///Use this method to get the current bot short description for the given user language. Returns [BotShortDescription](https://core.telegram.org/bots/api/#botshortdescription) on success.
#[derive(Clone)]
pub struct GetMyShortDescriptionRequest<'a> {
    api: &'a API,
    params: GetMyShortDescriptionParams,
}

impl<'a> RequestT for GetMyShortDescriptionRequest<'a> {
    type ParamsType = GetMyShortDescriptionParams;
    type ReturnType = BotShortDescription;
    fn get_name() -> &'static str {
        "getMyShortDescription"
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
impl<'a> GetMyShortDescriptionRequest<'a> {
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: GetMyShortDescriptionParams {
                language_code: Option::default(),
            },
        }
    }

    ///A two-letter ISO 639-1 language code or an empty string
    pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
        self.params.language_code = Some(language_code.into());
        self
    }
}

impl<'a> API {
    ///Use this method to get the current bot short description for the given user language. Returns [BotShortDescription](https://core.telegram.org/bots/api/#botshortdescription) on success.
    pub fn get_my_short_description(&'a self) -> GetMyShortDescriptionRequest {
        GetMyShortDescriptionRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
