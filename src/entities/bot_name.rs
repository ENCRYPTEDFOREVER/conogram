use serde::{Deserialize, Serialize};

/// This object represents the bot's name.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#botname)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotName {
    /// The bot's name
    pub name: String,
}

// Divider: all content below this line will be preserved after code regen
