use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A monowidth text.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextcode)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextCode {
    /// The text
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
