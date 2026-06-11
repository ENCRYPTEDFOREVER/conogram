use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A marked text.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextmarked)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextMarked {
    /// The text
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
