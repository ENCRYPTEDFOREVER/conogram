use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// This object contains information about the bot that was created to be managed by the current bot.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#managedbotcreated)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBotCreated {
    /// Information about the bot. The bot's token can be fetched using the method [getManagedBotToken](https://core.telegram.org/bots/api/#getmanagedbottoken).
    pub bot: User,
}

// Divider: all content below this line will be preserved after code regen
