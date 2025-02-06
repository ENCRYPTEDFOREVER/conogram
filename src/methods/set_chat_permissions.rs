use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{chat_permissions::ChatPermissions, misc::chat_id::ChatId},
    utils::deserialize_utils::is_false,
};

/// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the *can\_restrict\_members* administrator rights. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setchatpermissions)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetChatPermissionsParams {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatId,

    /// A JSON-serialized object for new default chat permissions
    pub permissions: ChatPermissions,

    /// Pass *True* if chat permissions are set independently. Otherwise, the *can\_send\_other\_messages* and *can\_add\_web\_page\_previews* permissions will imply the *can\_send\_messages*, *can\_send\_audios*, *can\_send\_documents*, *can\_send\_photos*, *can\_send\_videos*, *can\_send\_video\_notes*, and *can\_send\_voice\_notes* permissions; the *can\_send\_polls* permission will imply the *can\_send\_messages* permission.
    #[serde(skip_serializing_if = "is_false")]
    pub use_independent_chat_permissions: bool,
}

// Divider: all content below this line will be preserved after code regen
