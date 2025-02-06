use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setchatadministratorcustomtitle)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetChatAdministratorCustomTitleParams {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatId,

    /// Unique identifier of the target user
    pub user_id: i64,

    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub custom_title: String,
}

// Divider: all content below this line will be preserved after code regen
