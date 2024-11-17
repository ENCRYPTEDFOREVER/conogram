use serde::{Deserialize, Serialize};

/// Describes an inline message to be sent by a user of a Mini App.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#preparedinlinemessage)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PreparedInlineMessage {
    /// Unique identifier of the prepared message
    pub id: String,

    /// Expiration date of the prepared message, in Unix time. Expired prepared messages can no longer be used
    pub expiration_date: i64,
}

// Divider: all content below this line will be preserved after code regen
