use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{api::Api, errors::ConogramError, impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]
pub struct SetMyShortDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl_into_future!(SetMyShortDescriptionRequest<'a>);

///Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns *True* on success.
#[derive(Clone)]
pub struct SetMyShortDescriptionRequest<'a> {
    api: &'a Api,
    params: SetMyShortDescriptionParams,
}

impl RequestT for SetMyShortDescriptionRequest<'_> {
    type ParamsType = SetMyShortDescriptionParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setMyShortDescription"
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
impl<'a> SetMyShortDescriptionRequest<'a> {
    pub fn new(api: &'a Api) -> Self {
        Self {
            api,
            params: SetMyShortDescriptionParams {
                short_description: Option::default(),
                language_code: Option::default(),
            },
        }
    }

    ///New short description for the bot; 0-120 characters. Pass an empty string to remove the dedicated short description for the given language.
    #[must_use]
    pub fn short_description(mut self, short_description: impl Into<String>) -> Self {
        self.params.short_description = Some(short_description.into());
        self
    }

    ///A two-letter ISO 639-1 language code. If empty, the short description will be applied to all users for whose language there is no dedicated short description.
    #[must_use]
    pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
        self.params.language_code = Some(language_code.into());
        self
    }
}

impl Api {
    ///Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns *True* on success.
    pub fn set_my_short_description(&self) -> SetMyShortDescriptionRequest {
        SetMyShortDescriptionRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
