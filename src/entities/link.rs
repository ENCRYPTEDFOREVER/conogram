use serde::{Deserialize, Serialize};

/// Represents an HTTP link.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#link)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Link {
    /// URL of the link
    pub url: String,
}

// Divider: all content below this line will be preserved after code regen
