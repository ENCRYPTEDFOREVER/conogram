use serde::{Deserialize, Serialize};

///Represents the default [scope](https://core.telegram.org/bots/api/#botcommandscope) of bot commands. Default commands are used if no commands with a [narrower scope](https://core.telegram.org/bots/api/#determining-list-of-commands) are specified for the user.
///API Reference: [link](https://core.telegram.org/bots/api/#botcommandscopedefault)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeDefault {}
// Divider: all content below this line will be preserved after code regen
