use conogram_derives::Request;
use serde::Serialize;

use crate::entities::menu_button::MenuButton;

/// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setchatmenubutton)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetChatMenuButtonParams {
    /// Unique identifier for the target private chat. If not specified, default bot's menu button will be changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    /// A JSON-serialized object for the bot's new menu button. Defaults to [MenuButtonDefault](https://core.telegram.org/bots/api/#menubuttondefault)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<MenuButton>,
}

// Divider: all content below this line will be preserved after code regen
