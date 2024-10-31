use serde::{Deserialize, Serialize};

/// This object represents a unique message identifier.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#messageid)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageId {
    /// Unique message identifier. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    pub message_id: i64,
}

// Divider: all content below this line will be preserved after code regen
