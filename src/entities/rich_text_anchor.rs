use serde::{Deserialize, Serialize};

/// An anchor.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextanchor)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RichTextAnchor {
    /// The name of the anchor
    pub name: String,
}

// Divider: all content below this line will be preserved after code regen
