use serde::{Deserialize, Serialize};

use crate::entities::{location::Location, user::User};

/// Represents a [result](https://core.telegram.org/bots/api/#inlinequeryresult) of an inline query that was chosen by the user and sent to their chat partner.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#choseninlineresult)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,

    /// The user that chose the result
    pub from: User,

    /// *Optional*. Sender location, only for bots that require user location
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    /// *Optional*. Identifier of the sent inline message. Available only if there is an [inline keyboard](https://core.telegram.org/bots/api/#inlinekeyboardmarkup) attached to the message. Will be also received in [callback queries](https://core.telegram.org/bots/api/#callbackquery) and can be used to [edit](https://core.telegram.org/bots/api/#updating-messages) the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    /// The query that was used to obtain the result
    pub query: String,
}

// Divider: all content below this line will be preserved after code regen
