use serde::Serialize;

/// A divider, corresponding to the HTML tag `<hr/>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockdivider)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename = "divider", tag = "type")]
pub struct InputRichBlockDivider {}

// Divider: all content below this line will be preserved after code regen
