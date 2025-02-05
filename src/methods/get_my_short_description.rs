


use serde::Serialize;

use crate::{
    api::Api, entities::bot_short_description::BotShortDescription, 
    impl_into_future, request::RequestT,
};

#[derive(Debug, Clone, Serialize)]

pub struct GetMyShortDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl_into_future!(GetMyShortDescriptionRequest<'a>);

///Use this method to get the current bot short description for the given user language. Returns [BotShortDescription](https://core.telegram.org/bots/api/#botshortdescription) on success.
#[derive(Clone)]
pub struct GetMyShortDescriptionRequest<'a> {
    api: &'a Api,
    params: GetMyShortDescriptionParams,
}

impl RequestT for GetMyShortDescriptionRequest<'_> {
    type ParamsType = GetMyShortDescriptionParams;
    type ReturnType = BotShortDescription;
    fn get_name() -> &'static str {
        "getMyShortDescription"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> GetMyShortDescriptionRequest<'a> {
    pub fn new(api: &'a Api) -> Self {
        Self {
            api,
            params: GetMyShortDescriptionParams {
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
    ///Use this method to get the current bot short description for the given user language. Returns [BotShortDescription](https://core.telegram.org/bots/api/#botshortdescription) on success.
    pub fn get_my_short_description(&self) -> GetMyShortDescriptionRequest {
        GetMyShortDescriptionRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
