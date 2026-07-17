use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A hashtag.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtexthashtag)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hashtag", tag = "type")]
pub struct RichTextHashtag {
    /// The text
    pub text: Box<RichText>,

    /// The hashtag
    pub hashtag: String,
}

// Divider: all content below this line will be preserved after code regen
