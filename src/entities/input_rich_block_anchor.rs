use serde::Serialize;

/// A block with an anchor, corresponding to the HTML tag `<a>` with the attribute `name`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockanchor)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename = "anchor", tag = "type")]
pub struct InputRichBlockAnchor {
    /// The name of the anchor
    pub name: String,
}

// Divider: all content below this line will be preserved after code regen
