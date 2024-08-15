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
///
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
    pub const fn user(&self) -> &User {
        match self {
            Self::Owner(m) => &m.user,
            Self::Administrator(m) => &m.user,
            Self::Member(m) => &m.user,
            Self::Restricted(m) => &m.user,
            Self::Left(m) => &m.user,
            Self::Banned(m) => &m.user,
        }
    }

    pub const fn is_admin(&self) -> bool {
        matches!(self, Self::Owner(_) | Self::Administrator(_))
    }

    pub const fn is_in_chat(&self) -> bool {
        match self {
            Self::Owner(_) | Self::Administrator(_) | Self::Member(_) => true,
            Self::Restricted(m) => m.is_member,
            _ => false,
        }
    }

    pub const fn get_until_date(&self) -> Option<i64> {
        if let Self::Banned(m) = &self {
            Some(m.until_date)
        } else {
            None
        }
    }

    pub const fn is_banned(&self) -> bool {
        matches!(self, Self::Banned(_))
    }

    pub const fn is_restricted(&self) -> bool {
        matches!(self, Self::Restricted(_))
    }
}
