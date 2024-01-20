use serde::{Deserialize, Serialize};

use crate::entities::chat::Chat;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatId {
    String(String),
    Integer(i64),
}

impl ToString for ChatId {
    fn to_string(&self) -> String {
        match self {
            ChatId::String(value) => value.to_string(),
            ChatId::Integer(value) => value.to_string(),
        }
    }
}

impl From<i64> for ChatId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<&str> for ChatId {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<&String> for ChatId {
    fn from(value: &String) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for ChatId {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl<T: AsRef<Chat>> From<T> for ChatId {
    fn from(value: T) -> Self {
        Self::Integer(value.as_ref().id)
    }
}

impl Default for ChatId {
    fn default() -> Self {
        Self::Integer(0)
    }
}
