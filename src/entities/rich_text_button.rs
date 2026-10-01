use serde::{Deserialize, Serialize};

use crate::entities::rich_message_button::RichMessageButton;

/// A button.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextbutton)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "button", tag = "type")]
pub struct RichTextButton {
    /// The button
    pub button: RichMessageButton,
}

// Divider: all content below this line will be preserved after code regen
