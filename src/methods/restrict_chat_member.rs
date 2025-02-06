use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{chat_permissions::ChatPermissions, misc::chat_id::ChatId},
    utils::deserialize_utils::is_false,
};

/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass *True* for all permissions to lift restrictions from a user. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#restrictchatmember)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct RestrictChatMemberParams {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatId,

    /// Unique identifier of the target user
    pub user_id: i64,

    /// A JSON-serialized object for new user permissions
    pub permissions: ChatPermissions,

    /// Pass *True* if chat permissions are set independently. Otherwise, the *can\_send\_other\_messages* and *can\_add\_web\_page\_previews* permissions will imply the *can\_send\_messages*, *can\_send\_audios*, *can\_send\_documents*, *can\_send\_photos*, *can\_send\_videos*, *can\_send\_video\_notes*, and *can\_send\_voice\_notes* permissions; the *can\_send\_polls* permission will imply the *can\_send\_messages* permission.
    #[serde(skip_serializing_if = "is_false")]
    pub use_independent_chat_permissions: bool,

    /// Date when restrictions will be lifted for the user; Unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
