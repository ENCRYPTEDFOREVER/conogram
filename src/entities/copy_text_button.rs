use serde::{Deserialize, Serialize};

/// This object represents an inline keyboard button that copies specified text to the clipboard.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#copytextbutton)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CopyTextButton {
    /// The text to be copied to the clipboard; 1-256 characters
    pub text: String,
}

// Divider: all content below this line will be preserved after code regen
