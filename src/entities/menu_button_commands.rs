use serde::{Deserialize, Serialize};

/// Represents a menu button, which opens the bot's list of commands.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#menubuttoncommands)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MenuButtonCommands {}

// Divider: all content below this line will be preserved after code regen
