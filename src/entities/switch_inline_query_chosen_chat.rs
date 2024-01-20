use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.
///API Reference: [link](https://core.telegram.org/bots/api/#switchinlinequerychosenchat)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SwitchInlineQueryChosenChat {
    ///*Optional*. The default inline query to be inserted in the input field. If left empty, only the bot's username will be inserted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    ///*Optional*. True, if private chats with users can be chosen
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_user_chats: bool,

    ///*Optional*. True, if private chats with bots can be chosen
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_bot_chats: bool,

    ///*Optional*. True, if group and supergroup chats can be chosen
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_group_chats: bool,

    ///*Optional*. True, if channel chats can be chosen
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_channel_chats: bool,
}
// Divider: all content below this line will be preserved after code regen

impl<T: Into<Option<String>>> From<T> for SwitchInlineQueryChosenChat {
    fn from(value: T) -> Self {
        Self {
            query: value.into(),
            allow_user_chats: true,
            allow_bot_chats: true,
            allow_group_chats: true,
            allow_channel_chats: true,
        }
    }
}

impl SwitchInlineQueryChosenChat {
    pub fn new(
        query: impl Into<Option<String>>,
        allow_user_chats: bool,
        allow_bot_chats: bool,
        allow_group_chats: bool,
        allow_channel_chats: bool,
    ) -> Self {
        Self {
            query: query.into(),
            allow_user_chats,
            allow_bot_chats,
            allow_group_chats,
            allow_channel_chats,
        }
    }

    pub fn user_chats(query: impl Into<Option<String>>) -> Self {
        Self {
            query: query.into(),
            allow_user_chats: true,
            ..Default::default()
        }
    }

    pub fn bot_chats(query: impl Into<Option<String>>) -> Self {
        Self {
            query: query.into(),
            allow_bot_chats: true,
            ..Default::default()
        }
    }

    pub fn group_chats(query: impl Into<Option<String>>) -> Self {
        Self {
            query: query.into(),
            allow_group_chats: true,
            ..Default::default()
        }
    }

    pub fn channel_chats(query: impl Into<Option<String>>) -> Self {
        Self {
            query: query.into(),
            allow_channel_chats: true,
            ..Default::default()
        }
    }
}
