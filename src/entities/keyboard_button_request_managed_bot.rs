use serde::{Deserialize, Serialize};

/// This object defines the parameters for the creation of a managed bot. Information about the created bot will be shared with the bot using the update *managed\_bot* and a [Message](https://core.telegram.org/bots/api/#message) with the field *managed\_bot\_created*.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#keyboardbuttonrequestmanagedbot)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestManagedBot {
    /// Signed 32-bit identifier of the request. Must be unique within the message.
    pub request_id: i64,

    /// *Optional*. Suggested name for the bot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_name: Option<String>,

    /// *Optional*. Suggested username for the bot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_username: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
