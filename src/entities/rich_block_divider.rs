use serde::{Deserialize, Serialize};

/// A divider, corresponding to the HTML tag `<hr/>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockdivider)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "divider", tag = "type")]
pub struct RichBlockDivider {}

// Divider: all content below this line will be preserved after code regen
