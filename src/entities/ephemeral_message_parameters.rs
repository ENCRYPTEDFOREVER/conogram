use serde::{Deserialize, Serialize};

use crate::utils::deserialize_utils::is_false;

///
///
/// API Reference: [link](https://core.telegram.org/bots/api/#ephemeralmessageparameters)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EphemeralMessageParameters {
    /// Identifier of the user who will receive the message. It is not guaranteed that the user will receive the message, especially if they are offline. See [here](https://core.telegram.org/bots/api/#ephemeral-messages-and-commands) for more details.
    pub receiver_user_id: i64,

    /// *Optional*. Identifier of the callback query which triggered the message, if any
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_query_id: Option<String>,

    /// *Optional*. Pass *True* if the ephemeral message must be shown in place of the original message. Must be *False* for callback queries from ephemeral messages, which must be edited using regular *editEphemeralMessage…* methods.
    #[serde(default, skip_serializing_if = "is_false")]
    pub replace_callback_query_message: bool,
}

// Divider: all content below this line will be preserved after code regen
