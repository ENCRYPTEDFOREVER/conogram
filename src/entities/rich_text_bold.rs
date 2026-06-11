use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A bold text.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextbold)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextBold {
    /// The text
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
