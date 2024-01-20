use crate::entities::menu_button_commands::MenuButtonCommands;
use crate::entities::menu_button_default::MenuButtonDefault;
use crate::entities::menu_button_web_app::MenuButtonWebApp;
use serde::{Deserialize, Serialize};

///This object describes the bot's menu button in a private chat. It should be one of
///
///* [MenuButtonCommands](https://core.telegram.org/bots/api/#menubuttoncommands)
///* [MenuButtonWebApp](https://core.telegram.org/bots/api/#menubuttonwebapp)
///* [MenuButtonDefault](https://core.telegram.org/bots/api/#menubuttondefault)
///API Reference: [link](https://core.telegram.org/bots/api/#menubutton)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MenuButton {
    #[serde(rename = "commands")]
    Commands(MenuButtonCommands),
    #[serde(rename = "web_app")]
    WebApp(MenuButtonWebApp),
    #[serde(rename = "default")]
    Default(MenuButtonDefault),
}
impl Default for MenuButton {
    fn default() -> Self {
        Self::Commands(MenuButtonCommands::default())
    }
}
impl From<MenuButtonCommands> for MenuButton {
    fn from(value: MenuButtonCommands) -> Self {
        Self::Commands(value)
    }
}

impl From<MenuButtonWebApp> for MenuButton {
    fn from(value: MenuButtonWebApp) -> Self {
        Self::WebApp(value)
    }
}

impl From<MenuButtonDefault> for MenuButton {
    fn from(value: MenuButtonDefault) -> Self {
        Self::Default(value)
    }
}
// Divider: all content below this line will be preserved after code regen
