use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A text with a link.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtexturl)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextUrl {
    /// The text
    pub text: Box<RichText>,

    /// URL of the link
    pub url: String,
}

// Divider: all content below this line will be preserved after code regen
