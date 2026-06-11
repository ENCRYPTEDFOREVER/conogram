use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A link to an anchor.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextanchorlink)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextAnchorLink {
    /// The link text
    pub text: Box<RichText>,

    /// The name of the anchor. If the name is empty, then the link brings back to the top of the message.
    pub anchor_name: String,
}

// Divider: all content below this line will be preserved after code regen
