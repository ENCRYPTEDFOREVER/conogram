use crate::entities::inline_keyboard_button::InlineKeyboardButton;
use serde::{Deserialize, Serialize};

///This object represents an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) that appears right next to the message it belongs to.
///API Reference: [link](https://core.telegram.org/bots/api/#inlinekeyboardmarkup)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    ///Array of button rows, each represented by an Array of [InlineKeyboardButton](https://core.telegram.org/bots/api/#inlinekeyboardbutton) objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
// Divider: all content below this line will be preserved after code regen

impl InlineKeyboardMarkup {
    pub fn new(inline_keyboard: impl Into<Vec<Vec<InlineKeyboardButton>>>) -> Self {
        Self {
            inline_keyboard: inline_keyboard.into(),
        }
    }

    pub fn empty() -> Self {
        Self {
            inline_keyboard: vec![],
        }
    }
}
