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
pub struct SetMyDefaultAdministratorRightsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<ChatAdministratorRights>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub for_channels: bool,
}

impl_into_future!(SetMyDefaultAdministratorRightsRequest<'a>);

///Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns *True* on success.
#[derive(Clone)]
pub struct SetMyDefaultAdministratorRightsRequest<'a> {
    api: &'a API,
    params: SetMyDefaultAdministratorRightsParams,
}

impl<'a> RequestT for SetMyDefaultAdministratorRightsRequest<'a> {
    type ParamsType = SetMyDefaultAdministratorRightsParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setMyDefaultAdministratorRights"
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
impl<'a> SetMyDefaultAdministratorRightsRequest<'a> {
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: SetMyDefaultAdministratorRightsParams {
                rights: Option::default(),
                for_channels: bool::default(),
            },
        }
    }

    ///A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.
    pub fn rights(mut self, rights: impl Into<ChatAdministratorRights>) -> Self {
        self.params.rights = Some(rights.into());
        self
    }

    ///Pass *True* to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.
    pub fn for_channels(mut self, for_channels: impl Into<bool>) -> Self {
        self.params.for_channels = for_channels.into();
        self
    }
}

impl<'a> API {
    ///Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns *True* on success.
    pub fn set_my_default_administrator_rights(&'a self) -> SetMyDefaultAdministratorRightsRequest {
        SetMyDefaultAdministratorRightsRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
