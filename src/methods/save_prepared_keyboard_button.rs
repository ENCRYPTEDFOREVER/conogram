use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{
    keyboard_button::KeyboardButton, prepared_keyboard_button::PreparedKeyboardButton,
};

/// Stores a keyboard button that can be used by a user within a Mini App. Returns a [PreparedKeyboardButton](https://core.telegram.org/bots/api/#preparedkeyboardbutton) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#savepreparedkeyboardbutton)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = PreparedKeyboardButton)]
pub struct SavePreparedKeyboardButtonParams {
    /// Unique identifier of the target user that can use the button
    pub user_id: i64,

    /// A JSON-serialized object describing the button to be saved. The button must be of the type *request\_users*, *request\_chat*, or *request\_managed\_bot*.
    pub button: KeyboardButton,
}

// Divider: all content below this line will be preserved after code regen
