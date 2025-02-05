


use serde::Serialize;

use crate::{
    api::Api, entities::bot_description::BotDescription,  impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]

pub struct GetMyDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl_into_future!(GetMyDescriptionRequest<'a>);

///Use this method to get the current bot description for the given user language. Returns [BotDescription](https://core.telegram.org/bots/api/#botdescription) on success.
#[derive(Clone)]
pub struct GetMyDescriptionRequest<'a> {
    api: &'a Api,
    params: GetMyDescriptionParams,
}

impl RequestT for GetMyDescriptionRequest<'_> {
    type ParamsType = GetMyDescriptionParams;
    type ReturnType = BotDescription;
    fn get_name() -> &'static str {
        "getMyDescription"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> GetMyDescriptionRequest<'a> {
    pub fn new(api: &'a Api) -> Self {
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

impl Api {
    ///Use this method to get the current bot description for the given user language. Returns [BotDescription](https://core.telegram.org/bots/api/#botdescription) on success.
    pub fn get_my_description(&self) -> GetMyDescriptionRequest {
        GetMyDescriptionRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
