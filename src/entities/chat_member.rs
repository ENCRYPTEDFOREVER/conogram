use crate::entities::chat_member_administrator::ChatMemberAdministrator;
use crate::entities::chat_member_banned::ChatMemberBanned;
use crate::entities::chat_member_left::ChatMemberLeft;
use crate::entities::chat_member_member::ChatMemberMember;
use crate::entities::chat_member_owner::ChatMemberOwner;
use crate::entities::chat_member_restricted::ChatMemberRestricted;
use serde::{Deserialize, Serialize};

///This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:
///
///* [ChatMemberOwner](https://core.telegram.org/bots/api/#chatmemberowner)
///* [ChatMemberAdministrator](https://core.telegram.org/bots/api/#chatmemberadministrator)
///* [ChatMemberMember](https://core.telegram.org/bots/api/#chatmembermember)
///* [ChatMemberRestricted](https://core.telegram.org/bots/api/#chatmemberrestricted)
///* [ChatMemberLeft](https://core.telegram.org/bots/api/#chatmemberleft)
///* [ChatMemberBanned](https://core.telegram.org/bots/api/#chatmemberbanned)
///API Reference: [link](https://core.telegram.org/bots/api/#chatmember)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ChatMember {
    #[serde(rename = "creator")]
    Owner(ChatMemberOwner),
    #[serde(rename = "administrator")]
    Administrator(ChatMemberAdministrator),
    #[serde(rename = "member")]
    Member(ChatMemberMember),
    #[serde(rename = "restricted")]
    Restricted(ChatMemberRestricted),
    #[serde(rename = "left")]
    Left(ChatMemberLeft),
    #[serde(rename = "kicked")]
    Banned(ChatMemberBanned),
}
impl Default for ChatMember {
    fn default() -> Self {
        Self::Owner(ChatMemberOwner::default())
    }
}
impl From<ChatMemberOwner> for ChatMember {
    fn from(value: ChatMemberOwner) -> Self {
        Self::Owner(value)
    }
}

impl From<ChatMemberAdministrator> for ChatMember {
    fn from(value: ChatMemberAdministrator) -> Self {
        Self::Administrator(value)
    }
}

impl From<ChatMemberMember> for ChatMember {
    fn from(value: ChatMemberMember) -> Self {
        Self::Member(value)
    }
}

impl From<ChatMemberRestricted> for ChatMember {
    fn from(value: ChatMemberRestricted) -> Self {
        Self::Restricted(value)
    }
}

impl From<ChatMemberLeft> for ChatMember {
    fn from(value: ChatMemberLeft) -> Self {
        Self::Left(value)
    }
}

impl From<ChatMemberBanned> for ChatMember {
    fn from(value: ChatMemberBanned) -> Self {
        Self::Banned(value)
    }
}
// Divider: all content below this line will be preserved after code regen

use super::user::User;

impl ChatMember {
    /// Returns a User object from underlying value
    pub fn user(&self) -> &User {
        match self {
            ChatMember::Owner(m) => &m.user,
            ChatMember::Administrator(m) => &m.user,
            ChatMember::Member(m) => &m.user,
            ChatMember::Restricted(m) => &m.user,
            ChatMember::Left(m) => &m.user,
            ChatMember::Banned(m) => &m.user,
        }
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, ChatMember::Owner(_) | ChatMember::Administrator(_))
    }

    pub fn is_in_chat(&self) -> bool {
        match self {
            ChatMember::Owner(_) | ChatMember::Administrator(_) | ChatMember::Member(_) => true,
            ChatMember::Restricted(m) => m.is_member,
            _ => false,
        }
    }
}
