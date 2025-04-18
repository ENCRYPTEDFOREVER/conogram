use serde::{Deserialize, Serialize};

use crate::entities::{business_bot_rights::BusinessBotRights, user::User};

/// Describes the connection of the bot with a business account.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#businessconnection)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusinessConnection {
    /// Unique identifier of the business connection
    pub id: String,

    /// Business account user that created the business connection
    pub user: User,

    /// Identifier of a private chat with the user who created the business connection. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub user_chat_id: i64,

    /// Date the connection was established in Unix time
    pub date: i64,

    /// *Optional*. Rights of the business bot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rights: Option<BusinessBotRights>,

    /// True, if the connection is active
    pub is_enabled: bool,
}

// Divider: all content below this line will be preserved after code regen
