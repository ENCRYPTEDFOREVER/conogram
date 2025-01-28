use serde::{Deserialize, Serialize};

use crate::entities::{
    chat_member_administrator::ChatMemberAdministrator, chat_member_banned::ChatMemberBanned,
    chat_member_left::ChatMemberLeft, chat_member_member::ChatMemberMember,
    chat_member_owner::ChatMemberOwner, chat_member_restricted::ChatMemberRestricted,
};

/// This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:
///
/// * [ChatMemberOwner](https://core.telegram.org/bots/api/#chatmemberowner)
/// * [ChatMemberAdministrator](https://core.telegram.org/bots/api/#chatmemberadministrator)
/// * [ChatMemberMember](https://core.telegram.org/bots/api/#chatmembermember)
/// * [ChatMemberRestricted](https://core.telegram.org/bots/api/#chatmemberrestricted)
/// * [ChatMemberLeft](https://core.telegram.org/bots/api/#chatmemberleft)
/// * [ChatMemberBanned](https://core.telegram.org/bots/api/#chatmemberbanned)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatmember)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ChatMember {
    /// Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that owns the chat and has all administrator privileges.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#chatmemberowner)
    #[serde(rename = "creator")]
    Owner(ChatMemberOwner),

    /// Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that has some additional privileges.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#chatmemberadministrator)
    #[serde(rename = "administrator")]
    Administrator(ChatMemberAdministrator),

    /// Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that has no additional privileges or restrictions.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#chatmembermember)
    #[serde(rename = "member")]
    Member(ChatMemberMember),

    /// Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that is under certain restrictions in the chat. Supergroups only.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#chatmemberrestricted)
    #[serde(rename = "restricted")]
    Restricted(ChatMemberRestricted),

    /// Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that isn't currently a member of the chat, but may join it themselves.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#chatmemberleft)
    #[serde(rename = "left")]
    Left(ChatMemberLeft),

    /// Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that was banned in the chat and can't return to the chat or view chat messages.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#chatmemberbanned)
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
    #[must_use]
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

    #[must_use]
    pub const fn is_admin(&self) -> bool {
        matches!(self, Self::Owner(_) | Self::Administrator(_))
    }

    #[must_use]
    pub const fn is_in_chat(&self) -> bool {
        match self {
            Self::Owner(_) | Self::Administrator(_) | Self::Member(_) => true,
            Self::Restricted(m) => m.is_member,
            _ => false,
        }
    }

    #[must_use]
    pub const fn get_until_date(&self) -> Option<i64> {
        if let Self::Banned(m) = &self {
            Some(m.until_date)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn is_banned(&self) -> bool {
        matches!(self, Self::Banned(_))
    }

    #[must_use]
    pub const fn is_restricted(&self) -> bool {
        matches!(self, Self::Restricted(_))
    }
}
