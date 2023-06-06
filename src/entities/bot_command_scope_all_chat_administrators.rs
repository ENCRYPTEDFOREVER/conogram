use serde::{Deserialize, Serialize};

///Represents the [scope](https://core.telegram.org/bots/api/#botcommandscope) of bot commands, covering all group and supergroup chat administrators.
///API Reference: [link](https://core.telegram.org/bots/api/#botcommandscopeallchatadministrators)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {}
// Divider: all content below this line will be preserved after code regen
