use serde::{Deserialize, Serialize};

/// Represents the [scope](https://core.telegram.org/bots/api/#botcommandscope) of bot commands, covering all group and supergroup chats.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#botcommandscopeallgroupchats)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "all_group_chats", tag = "type")]
pub struct BotCommandScopeAllGroupChats {}

// Divider: all content below this line will be preserved after code regen
