use crate::api::API;
use crate::entities::passport_element_error::PassportElementError;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetPassportDataErrorsParams {
    pub user_id: i64,
    pub errors: Vec<PassportElementError>,
}

impl_into_future!(SetPassportDataErrorsRequest<'a>);

///Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns *True* on success.
///
///Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
#[derive(Clone)]
pub struct SetPassportDataErrorsRequest<'a> {
    api: &'a API,
    params: SetPassportDataErrorsParams,
}

impl<'a> RequestT for SetPassportDataErrorsRequest<'a> {
    type ParamsType = SetPassportDataErrorsParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setPassportDataErrors"
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
impl<'a> SetPassportDataErrorsRequest<'a> {
    pub fn new(api: &'a API, user_id: i64, errors: Vec<PassportElementError>) -> Self {
        Self {
            api,
            params: SetPassportDataErrorsParams { user_id, errors },
        }
    }

    ///User identifier
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///A JSON-serialized array describing the errors
    pub fn errors(mut self, errors: impl Into<Vec<PassportElementError>>) -> Self {
        self.params.errors = errors.into();
        self
    }
}

impl<'a> API {
    ///Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns *True* on success.
    ///
    ///Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
    pub fn set_passport_data_errors(
        &'a self,
        user_id: impl Into<i64>,
        errors: impl Into<Vec<PassportElementError>>,
    ) -> SetPassportDataErrorsRequest {
        SetPassportDataErrorsRequest::new(self, user_id.into(), errors.into())
    }
}

// Divider: all content below this line will be preserved after code regen
