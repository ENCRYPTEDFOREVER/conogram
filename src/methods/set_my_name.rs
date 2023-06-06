use crate::api::API;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetMyNameParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl_into_future!(SetMyNameRequest<'a>);

///Use this method to change the bot's name. Returns *True* on success.
#[derive(Clone)]
pub struct SetMyNameRequest<'a> {
    api: &'a API,
    params: SetMyNameParams,
}

impl<'a> RequestT for SetMyNameRequest<'a> {
    type ParamsType = SetMyNameParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setMyName"
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
impl<'a> SetMyNameRequest<'a> {
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: SetMyNameParams {
                name: Option::default(),
                language_code: Option::default(),
            },
        }
    }

    ///New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    ///A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name.
    pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
        self.params.language_code = Some(language_code.into());
        self
    }
}

impl<'a> API {
    ///Use this method to change the bot's name. Returns *True* on success.
    pub fn set_my_name(&'a self) -> SetMyNameRequest {
        SetMyNameRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
