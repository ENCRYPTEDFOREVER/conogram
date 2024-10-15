use serde::{Deserialize, Serialize};

use crate::utils::deserialize_utils::is_false;

/// This object represents a Telegram user or bot.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#user)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,

    /// *True*, if this user is a bot
    pub is_bot: bool,

    /// User's or bot's first name
    pub first_name: String,

    /// *Optional*. User's or bot's last name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// *Optional*. User's or bot's username
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// *Optional*. [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// *Optional*. *True*, if this user is a Telegram Premium user
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_premium: bool,

    /// *Optional*. *True*, if this user added the bot to the attachment menu
    #[serde(default, skip_serializing_if = "is_false")]
    pub added_to_attachment_menu: bool,

    /// *Optional*. *True*, if the bot can be invited to groups. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_join_groups: bool,

    /// *Optional*. *True*, if [privacy mode](https://core.telegram.org/bots/features#privacy-mode) is disabled for the bot. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_read_all_group_messages: bool,

    /// *Optional*. *True*, if the bot supports inline queries. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).
    #[serde(default, skip_serializing_if = "is_false")]
    pub supports_inline_queries: bool,

    /// *Optional*. *True*, if the bot can be connected to a Telegram Business account to receive its messages. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_connect_to_business: bool,

    /// *Optional*. *True*, if the bot has a main Web App. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_main_web_app: bool,
}

// Divider: all content below this line will be preserved after code regen

impl User {
    pub fn get_url(&self) -> String {
        if let Some(username) = &self.username {
            format!("https://t.me/{username}")
        } else {
            // Will only work in bot api
            format!("tg://user?id={}", self.id)
        }
    }

    pub fn full_name(&self) -> String {
        match &self.last_name {
            Some(last) => format!("{} {}", self.first_name, last),
            None => self.first_name.clone(),
        }
    }

    pub fn mention_html(&self) -> String {
        if let Some(username) = &self.username {
            format!("@{username}")
        } else {
            format!(
                "<a href=\"tg://user?id={}\">{}</a>",
                self.id,
                self.full_name()
            )
        }
    }
}
