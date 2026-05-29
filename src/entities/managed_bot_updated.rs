use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// This object contains information about the creation, token update, or owner update of a bot that is managed by the current bot.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#managedbotupdated)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBotUpdated {
    /// User that created the bot
    pub user: User,

    /// Information about the bot. Token of the bot can be fetched using the method [getManagedBotToken](https://core.telegram.org/bots/api/#getmanagedbottoken).
    pub bot: User,
}

// Divider: all content below this line will be preserved after code regen
