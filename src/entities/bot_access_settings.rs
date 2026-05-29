use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// This object describes the access settings of a bot.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#botaccesssettings)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotAccessSettings {
    /// *True*, if only selected users can access the bot. The bot's owner can always access it.
    pub is_access_restricted: bool,

    /// *Optional*. The list of other users who have access to the bot if the access is restricted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added_users: Vec<User>,
}

// Divider: all content below this line will be preserved after code regen
