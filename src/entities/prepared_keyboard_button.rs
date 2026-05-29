use serde::{Deserialize, Serialize};

/// Describes a keyboard button to be used by a user of a Mini App.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#preparedkeyboardbutton)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreparedKeyboardButton {
    /// Unique identifier of the keyboard button
    pub id: String,
}

// Divider: all content below this line will be preserved after code regen
