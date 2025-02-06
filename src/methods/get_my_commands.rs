use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{bot_command::BotCommand, bot_command_scope::BotCommandScope};

/// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of [BotCommand](https://core.telegram.org/bots/api/#botcommand) objects. If commands aren't set, an empty list is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getmycommands)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Vec<BotCommand>)]
pub struct GetMyCommandsParams {
    /// A JSON-serialized object, describing scope of users. Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api/#botcommandscopedefault).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,

    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
