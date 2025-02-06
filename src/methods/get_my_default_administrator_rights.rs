use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::chat_administrator_rights::ChatAdministratorRights,
    utils::deserialize_utils::is_false,
};

/// Use this method to get the current default administrator rights of the bot. Returns [ChatAdministratorRights](https://core.telegram.org/bots/api/#chatadministratorrights) on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getmydefaultadministratorrights)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = ChatAdministratorRights)]
pub struct GetMyDefaultAdministratorRightsParams {
    /// Pass *True* to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.
    #[serde(skip_serializing_if = "is_false")]
    pub for_channels: bool,
}

// Divider: all content below this line will be preserved after code regen
