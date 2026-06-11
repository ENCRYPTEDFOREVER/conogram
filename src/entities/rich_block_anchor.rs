use serde::{Deserialize, Serialize};

/// A block with an anchor, corresponding to the HTML tag `<a>` with the attribute `name`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockanchor)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RichBlockAnchor {
    /// The name of the anchor
    pub name: String,
}

// Divider: all content below this line will be preserved after code regen
