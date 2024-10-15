use serde::{Deserialize, Serialize};

/// This object represents a unique message identifier.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#messageid)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageId {
    /// Unique message identifier
    pub message_id: i64,
}

// Divider: all content below this line will be preserved after code regen
