use serde::{Deserialize, Serialize};

/// Represents the [scope](https://core.telegram.org/bots/api/#botcommandscope) of bot commands, covering all private chats.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#botcommandscopeallprivatechats)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotCommandScopeAllPrivateChats {}

// Divider: all content below this line will be preserved after code regen
