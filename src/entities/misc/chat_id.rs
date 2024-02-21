use serde::{Deserialize, Serialize};

use crate::entities::chat::Chat;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ChatId {
    /// @channel_username
    Username(String),

    /// Chat/Channel ID
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

impl ToString for ChatId {
    fn to_string(&self) -> String {
        match self {
            Self::Username(value) => value.to_string(),
            Self::Id(value) => value.to_string(),
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
