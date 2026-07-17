use serde::{Deserialize, Serialize};

use crate::utils::deserialize_utils::is_false;

/// This object represents a bot command.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#botcommand)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotCommand {
    /// Text of the command; 1-32 characters. Can contain only lowercase English letters, digits and underscores.
    pub command: String,

    /// Description of the command; 1-256 characters
    pub description: String,

    /// *Optional*. *True*, if the command sends an ephemeral message, which can be seen only by the sender of the message and the bot
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_ephemeral: bool,
}

// Divider: all content below this line will be preserved after code regen
