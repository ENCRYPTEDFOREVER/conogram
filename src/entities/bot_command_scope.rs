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
///
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
impl Default for BotCommandScope {
    fn default() -> Self {
        Self::Default(BotCommandScopeDefault::default())
    }
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
use std::fmt::Display;

impl Display for BotCommandScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Default(_) => "default",
            Self::AllPrivateChats(_) => "all_private_chats",
            Self::AllGroupChats(_) => "all_group_chats",
            Self::AllChatAdministrators(_) => "all_chat_administrators",
            Self::Chat(_) => "chat",
            Self::ChatAdministrators(_) => "chat_administrators",
            Self::ChatMember(_) => "chat_member",
        })
    }
}

use super::misc::chat_id::ChatId;
impl BotCommandScope {
    pub fn chat(chat_id: impl Into<ChatId>) -> Self {
        BotCommandScopeChat {
            chat_id: chat_id.into(),
        }
        .into()
    }

    pub fn chat_member(chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
        BotCommandScopeChatMember {
            chat_id: chat_id.into(),
            user_id: user_id.into(),
        }
        .into()
    }

    pub fn chat_administrators(chat_id: impl Into<ChatId>) -> Self {
        BotCommandScopeChatAdministrators {
            chat_id: chat_id.into(),
        }
        .into()
    }

    pub fn all_private_chats() -> Self {
        BotCommandScopeAllPrivateChats {}.into()
    }

    pub fn all_group_chats() -> Self {
        BotCommandScopeAllGroupChats {}.into()
    }

    pub fn all_chat_administrators() -> Self {
        BotCommandScopeAllChatAdministrators {}.into()
    }
}
