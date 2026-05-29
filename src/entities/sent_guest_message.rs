use serde::{Deserialize, Serialize};

/// Describes an inline message sent by a guest bot.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#sentguestmessage)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SentGuestMessage {
    /// Identifier of the sent inline message
    pub inline_message_id: String,
}

// Divider: all content below this line will be preserved after code regen
