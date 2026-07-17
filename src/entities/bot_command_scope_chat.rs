use serde::{Deserialize, Serialize};

use crate::entities::misc::chat_id::ChatId;

/// Represents the [scope](https://core.telegram.org/bots/api/#botcommandscope) of bot commands, covering a specific chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#botcommandscopechat)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "chat", tag = "type")]
pub struct BotCommandScopeChat {
    /// Unique identifier for the target chat or username of the target supergroup in the format `@username`. Channel direct messages chats and channel chats aren't supported.
    pub chat_id: ChatId,
}

// Divider: all content below this line will be preserved after code regen
