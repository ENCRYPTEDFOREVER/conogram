use serde::{Deserialize, Serialize};

use crate::utils::deserialize_utils::is_false;

///
///
/// API Reference: [link](https://core.telegram.org/bots/api/#ephemeralmessageparameters)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EphemeralMessageParameters {
    /// *Optional*. Identifier of the user who will receive the message; required unless the message is sent as an ephemeral reply to a callback query. If omitted, then the parameter *chat\_id* of the method is ignored. If specified together with *callback\_query\_id*, then it must be the identifier of the user who pressed the button. Must be omitted for messages sent in chats where the bot isn't a member, unless the message is a reply to an ephemeral message. It is not guaranteed that the user will receive the message, especially if they are offline. See [here](https://core.telegram.org/bots/api/#ephemeral-messages-and-commands) for more details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_user_id: Option<i64>,

    /// *Optional*. Identifier of the callback query which triggered the message, if any. If specified and *receiver\_user\_id* is omitted, then the message is sent as an ephemeral reply to the callback query and will be shown only to the user who pressed the button. Ephemeral replies can't be sent to callback queries from messages in channel direct messages chats. The bot must still answer the callback query using [answerCallbackQuery](https://core.telegram.org/bots/api/#answercallbackquery).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_query_id: Option<String>,

    /// *Optional*. Pass *True* if the ephemeral message must be shown in place of the original message. Must be *False* for callback queries from ephemeral messages, which must be edited using regular *editEphemeralMessage…* methods.
    #[serde(default, skip_serializing_if = "is_false")]
    pub replace_callback_query_message: bool,
}

// Divider: all content below this line will be preserved after code regen
