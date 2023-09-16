use crate::entities::bot_command_scope_all_chat_administrators::BotCommandScopeAllChatAdministrators;
use crate::entities::bot_command_scope_all_group_chats::BotCommandScopeAllGroupChats;
use crate::entities::bot_command_scope_all_private_chats::BotCommandScopeAllPrivateChats;
use crate::entities::bot_command_scope_chat::BotCommandScopeChat;
use crate::entities::bot_command_scope_chat_administrators::BotCommandScopeChatAdministrators;
use crate::entities::bot_command_scope_chat_member::BotCommandScopeChatMember;
use crate::entities::bot_command_scope_default::BotCommandScopeDefault;
use serde::{Deserialize, Serialize};

///This object represents the scope to which bot commands are applied. Currently, the following 7 scopes are supported:
///
///* [BotCommandScopeDefault](https://core.telegram.org/bots/api/#botcommandscopedefault)
///* [BotCommandScopeAllPrivateChats](https://core.telegram.org/bots/api/#botcommandscopeallprivatechats)
///* [BotCommandScopeAllGroupChats](https://core.telegram.org/bots/api/#botcommandscopeallgroupchats)
///* [BotCommandScopeAllChatAdministrators](https://core.telegram.org/bots/api/#botcommandscopeallchatadministrators)
///* [BotCommandScopeChat](https://core.telegram.org/bots/api/#botcommandscopechat)
///* [BotCommandScopeChatAdministrators](https://core.telegram.org/bots/api/#botcommandscopechatadministrators)
///* [BotCommandScopeChatMember](https://core.telegram.org/bots/api/#botcommandscopechatmember)
///API Reference: [link](https://core.telegram.org/bots/api/#botcommandscope)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BotCommandScope {
    #[serde(rename = "default")]
    Default(BotCommandScopeDefault),
    #[serde(rename = "all_private_chats")]
    AllPrivateChats(BotCommandScopeAllPrivateChats),
    #[serde(rename = "all_group_chats")]
    AllGroupChats(BotCommandScopeAllGroupChats),
    #[serde(rename = "all_chat_administrators")]
    AllChatAdministrators(BotCommandScopeAllChatAdministrators),
    #[serde(rename = "chat")]
    Chat(BotCommandScopeChat),
    #[serde(rename = "chat_administrators")]
    ChatAdministrators(BotCommandScopeChatAdministrators),
    #[serde(rename = "chat_member")]
    ChatMember(BotCommandScopeChatMember),
}
impl From<BotCommandScopeDefault> for BotCommandScope {
    fn from(value: BotCommandScopeDefault) -> Self {
        Self::Default(value)
    }
}

impl From<BotCommandScopeAllPrivateChats> for BotCommandScope {
    fn from(value: BotCommandScopeAllPrivateChats) -> Self {
        Self::AllPrivateChats(value)
    }
}

impl From<BotCommandScopeAllGroupChats> for BotCommandScope {
    fn from(value: BotCommandScopeAllGroupChats) -> Self {
        Self::AllGroupChats(value)
    }
}

impl From<BotCommandScopeAllChatAdministrators> for BotCommandScope {
    fn from(value: BotCommandScopeAllChatAdministrators) -> Self {
        Self::AllChatAdministrators(value)
    }
}

impl From<BotCommandScopeChat> for BotCommandScope {
    fn from(value: BotCommandScopeChat) -> Self {
        Self::Chat(value)
    }
}

impl From<BotCommandScopeChatAdministrators> for BotCommandScope {
    fn from(value: BotCommandScopeChatAdministrators) -> Self {
        Self::ChatAdministrators(value)
    }
}

impl From<BotCommandScopeChatMember> for BotCommandScope {
    fn from(value: BotCommandScopeChatMember) -> Self {
        Self::ChatMember(value)
    }
}
// Divider: all content below this line will be preserved after code regen

impl ToString for BotCommandScope {
    fn to_string(&self) -> String {
        match self {
            BotCommandScope::Default(_) => "default",
            BotCommandScope::AllPrivateChats(_) => "all_private_chats",
            BotCommandScope::AllGroupChats(_) => "all_group_chats",
            BotCommandScope::AllChatAdministrators(_) => "all_chat_administrators",
            BotCommandScope::Chat(_) => "chat",
            BotCommandScope::ChatAdministrators(_) => "chat_administrators",
            BotCommandScope::ChatMember(_) => "ChatMember",
        }
        .to_string()
    }
}

use super::misc::chat_id::ChatId;
impl BotCommandScope {
    pub fn chat(chat_id: impl Into<ChatId>) -> Self {
        Self::Chat(BotCommandScopeChat {
            chat_id: chat_id.into(),
        })
    }

    pub fn all_private_chats() -> Self {
        Self::AllPrivateChats(BotCommandScopeAllPrivateChats {})
    }
}
