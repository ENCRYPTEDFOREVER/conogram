use crate::api::API;
use crate::entities::chat_administrator_rights::ChatAdministratorRights;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct GetMyDefaultAdministratorRightsParams {
    #[serde(skip_serializing_if = "is_false", default)]
    pub for_channels: bool,
}

impl_into_future!(GetMyDefaultAdministratorRightsRequest<'a>);

///Use this method to get the current default administrator rights of the bot. Returns [ChatAdministratorRights](https://core.telegram.org/bots/api/#chatadministratorrights) on success.
#[derive(Clone)]
pub struct GetMyDefaultAdministratorRightsRequest<'a> {
    api: &'a API,
    params: GetMyDefaultAdministratorRightsParams,
}

impl<'a> RequestT for GetMyDefaultAdministratorRightsRequest<'a> {
    type ParamsType = GetMyDefaultAdministratorRightsParams;
    type ReturnType = ChatAdministratorRights;
    fn get_name() -> &'static str {
        "getMyDefaultAdministratorRights"
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
impl<'a> GetMyDefaultAdministratorRightsRequest<'a> {
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: GetMyDefaultAdministratorRightsParams {
                for_channels: bool::default(),
            },
        }
    }

    ///Pass *True* to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.
    pub fn for_channels(mut self, for_channels: impl Into<bool>) -> Self {
        self.params.for_channels = for_channels.into();
        self
    }
}

impl<'a> API {
    ///Use this method to get the current default administrator rights of the bot. Returns [ChatAdministratorRights](https://core.telegram.org/bots/api/#chatadministratorrights) on success.
    pub fn get_my_default_administrator_rights(&'a self) -> GetMyDefaultAdministratorRightsRequest {
        GetMyDefaultAdministratorRightsRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
