use conogram_derives::Request;
use serde::Serialize;

use crate::entities::menu_button::MenuButton;

/// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns [MenuButton](https://core.telegram.org/bots/api/#menubutton) on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getchatmenubutton)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = MenuButton)]
pub struct GetChatMenuButtonParams {
    /// Unique identifier for the target private chat. If not specified, default bot's menu button will be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
