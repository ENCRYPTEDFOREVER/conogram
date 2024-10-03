use serde::{Deserialize, Serialize};

use crate::entities::{chat::Chat, user::User};

/// This object represents an answer of a user in a non-anonymous poll.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#pollanswer)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,

    /// *Optional*. The chat that changed the answer to the poll, if the voter is anonymous
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voter_chat: Option<Box<Chat>>,

    /// *Optional*. The user that changed the answer to the poll, if the voter isn't anonymous
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// 0-based identifiers of chosen answer options. May be empty if the vote was retracted.
    pub option_ids: Vec<i64>,
}

// Divider: all content below this line will be preserved after code regen
