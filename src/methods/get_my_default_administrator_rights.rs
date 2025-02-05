


use serde::Serialize;

use crate::{
    api::Api, entities::chat_administrator_rights::ChatAdministratorRights, 
    impl_into_future, request::RequestT, utils::deserialize_utils::is_false,
};

#[derive(Debug, Clone, Serialize)]

pub struct GetMyDefaultAdministratorRightsParams {
    #[serde(default, skip_serializing_if = "is_false")]
    pub for_channels: bool,
}

impl_into_future!(GetMyDefaultAdministratorRightsRequest<'a>);

///Use this method to get the current default administrator rights of the bot. Returns [ChatAdministratorRights](https://core.telegram.org/bots/api/#chatadministratorrights) on success.
#[derive(Clone)]
pub struct GetMyDefaultAdministratorRightsRequest<'a> {
    api: &'a Api,
    params: GetMyDefaultAdministratorRightsParams,
}

impl RequestT for GetMyDefaultAdministratorRightsRequest<'_> {
    type ParamsType = GetMyDefaultAdministratorRightsParams;
    type ReturnType = ChatAdministratorRights;
    fn get_name() -> &'static str {
        "getMyDefaultAdministratorRights"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> GetMyDefaultAdministratorRightsRequest<'a> {
    pub fn new(api: &'a Api) -> Self {
        Self {
            api,
            params: GetMyDefaultAdministratorRightsParams {
                for_channels: bool::default(),
            },
        }
    }

    ///Pass *True* to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.
    #[must_use]
    pub fn for_channels(mut self, for_channels: impl Into<bool>) -> Self {
        self.params.for_channels = for_channels.into();
        self
    }
}

impl Api {
    ///Use this method to get the current default administrator rights of the bot. Returns [ChatAdministratorRights](https://core.telegram.org/bots/api/#chatadministratorrights) on success.
    pub fn get_my_default_administrator_rights(&self) -> GetMyDefaultAdministratorRightsRequest {
        GetMyDefaultAdministratorRightsRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
