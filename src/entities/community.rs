use serde::{Deserialize, Serialize};

/// Represents a community (a group of chats).
///
/// API Reference: [link](https://core.telegram.org/bots/api/#community)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Community {
    /// Unique identifier for this community. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,

    /// Name of the community
    pub name: String,
}

// Divider: all content below this line will be preserved after code regen
