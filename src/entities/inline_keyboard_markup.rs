use crate::entities::inline_keyboard_button::InlineKeyboardButton;
use serde::{Deserialize, Serialize};

/// This object represents an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) that appears right next to the message it belongs to.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inlinekeyboardmarkup)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of [InlineKeyboardButton](https://core.telegram.org/bots/api/#inlinekeyboardbutton) objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

// Divider: all content below this line will be preserved after code regen

impl InlineKeyboardMarkup {
    pub fn new(inline_keyboard: impl Into<Vec<Vec<InlineKeyboardButton>>>) -> Self {
        Self {
            inline_keyboard: inline_keyboard.into(),
        }
    }

    pub const fn empty() -> Self {
        Self {
            inline_keyboard: vec![],
        }
    }

    // Adds empty row to the keyboard
    #[must_use]
    pub fn add_row(mut self) -> Self {
        self.inline_keyboard.push(vec![]);
        self
    }

    // Adds a button to the last row of the keyboard. New row will be created if the keyboard is empty
    #[must_use]
    pub fn add_button(mut self, button: impl Into<InlineKeyboardButton>) -> Self {
        if self.inline_keyboard.is_empty() {
            self.inline_keyboard.push(Vec::with_capacity(1));
        }
        if let Some(row) = self.inline_keyboard.last_mut() {
            row.push(button.into());
        }
        self
    }

    // Adds a button to the new row
    #[must_use]
    pub fn add_button_row(self, button: impl Into<InlineKeyboardButton>) -> Self {
        self.add_row().add_button(button)
    }
}
