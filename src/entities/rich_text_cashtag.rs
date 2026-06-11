use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A cashtag.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextcashtag)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextCashtag {
    /// The text
    pub text: Box<RichText>,

    /// The cashtag
    pub cashtag: String,
}

// Divider: all content below this line will be preserved after code regen
