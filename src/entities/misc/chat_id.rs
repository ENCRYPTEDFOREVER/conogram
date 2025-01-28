use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::entities::chat::Chat;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ChatId {
    /// Chat/Channel username, for example: ``@BotNews``
    Username(String),

    /// Chat/Channel ID, for example: ``-1001004276399``, ``36265675``
    Id(i64),
}

impl ChatId {
    fn parse_from_str(value: &str) -> Self {
        if value.starts_with('@') {
            Self::Username(value.to_owned())
        } else {
            Self::Username(format!("@{value}"))
        }
    }
}

impl Display for ChatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Username(value) => f.write_str(value),
            Self::Id(value) => f.write_str(&value.to_string()),
        }
    }
}

impl From<i64> for ChatId {
    fn from(value: i64) -> Self {
        Self::Id(value)
    }
}

impl From<&str> for ChatId {
    fn from(value: &str) -> Self {
        Self::parse_from_str(value)
    }
}

impl From<&String> for ChatId {
    fn from(value: &String) -> Self {
        Self::parse_from_str(value)
    }
}

impl From<String> for ChatId {
    fn from(value: String) -> Self {
        Self::parse_from_str(&value)
    }
}

impl<T: AsRef<Chat>> From<T> for ChatId {
    fn from(value: T) -> Self {
        Self::Id(value.as_ref().id)
    }
}

impl Default for ChatId {
    fn default() -> Self {
        Self::Id(0)
    }
}
