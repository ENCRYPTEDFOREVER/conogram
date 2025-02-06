use conogram_derives::Request;
use serde::Serialize;

use crate::{entities::message::Message, utils::deserialize_utils::is_false};

/// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Returns an error, if the new score is not greater than the user's current score in the chat and *force* is *False*.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setgamescore)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Option<Message>)]
pub struct SetGameScoreParams {
    /// User identifier
    pub user_id: i64,

    /// New score, must be non-negative
    pub score: i64,

    /// Pass *True* if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    #[serde(skip_serializing_if = "is_false")]
    pub force: bool,

    /// Pass *True* if the game message should not be automatically edited to include the current scoreboard
    #[serde(skip_serializing_if = "is_false")]
    pub disable_edit_message: bool,

    /// Required if *inline\_message\_id* is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    /// Required if *inline\_message\_id* is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    /// Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
