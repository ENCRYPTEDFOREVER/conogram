


use serde::Serialize;

use crate::{api::Api,  impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]

pub struct SetMyDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl_into_future!(SetMyDescriptionRequest<'a>);

///Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns *True* on success.
#[derive(Clone)]
pub struct SetMyDescriptionRequest<'a> {
    api: &'a Api,
    params: SetMyDescriptionParams,
}

impl RequestT for SetMyDescriptionRequest<'_> {
    type ParamsType = SetMyDescriptionParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setMyDescription"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> SetMyDescriptionRequest<'a> {
    pub fn new(api: &'a Api) -> Self {
        Self {
            api,
            params: SetMyDescriptionParams {
                description: Option::default(),
                language_code: Option::default(),
            },
        }
    }

    ///New bot description; 0-512 characters. Pass an empty string to remove the dedicated description for the given language.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.params.description = Some(description.into());
        self
    }

    ///A two-letter ISO 639-1 language code. If empty, the description will be applied to all users for whose language there is no dedicated description.
    #[must_use]
    pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
        self.params.language_code = Some(language_code.into());
        self
    }
}

impl Api {
    ///Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns *True* on success.
    pub fn set_my_description(&self) -> SetMyDescriptionRequest {
        SetMyDescriptionRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
