use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::chat_administrator_rights::ChatAdministratorRights,
    utils::deserialize_utils::is_false,
};

/// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setmydefaultadministratorrights)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetMyDefaultAdministratorRightsParams {
    /// A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<ChatAdministratorRights>,

    /// Pass *True* to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.
    #[serde(skip_serializing_if = "is_false")]
    pub for_channels: bool,
}

// Divider: all content below this line will be preserved after code regen
