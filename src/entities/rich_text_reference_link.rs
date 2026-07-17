use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A link to a reference.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextreferencelink)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "reference_link", tag = "type")]
pub struct RichTextReferenceLink {
    /// The link text
    pub text: Box<RichText>,

    /// The name of the reference
    pub reference_name: String,
}

// Divider: all content below this line will be preserved after code regen
