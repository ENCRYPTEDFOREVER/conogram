use crate::entities::misc::chat_id::ChatId;
use serde::{Deserialize, Serialize};

///Represents the [scope](https://core.telegram.org/bots/api/#botcommandscope) of bot commands, covering all administrators of a specific group or supergroup chat.
///API Reference: [link](https://core.telegram.org/bots/api/#botcommandscopechatadministrators)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChatAdministrators {
    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatId,
}
// Divider: all content below this line will be preserved after code regen
