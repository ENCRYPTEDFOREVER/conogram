use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{bot_command::BotCommand, bot_command_scope::BotCommandScope};

/// Use this method to change the list of the bot's commands. See [this manual](https://core.telegram.org/bots/features#commands) for more details about bot commands. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setmycommands)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetMyCommandsParams {
    /// A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
    pub commands: Vec<BotCommand>,

    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api/#botcommandscopedefault).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,

    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
