use crate::entities::chat::Chat;
use crate::entities::user::User;
use crate::utils::deserialize_utils::deserialize_boxed_option;
use serde::{Deserialize, Serialize};

///This object represents an answer of a user in a non-anonymous poll.
///API Reference: [link](https://core.telegram.org/bots/api/#pollanswer)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PollAnswer {
    ///Unique poll identifier
    pub poll_id: String,

    ///*Optional*. The chat that changed the answer to the poll, if the voter is anonymous
    #[serde(
        deserialize_with = "deserialize_boxed_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub voter_chat: Option<Box<Chat>>,

    ///*Optional*. The user that changed the answer to the poll, if the voter isn't anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    ///0-based identifiers of chosen answer options. May be empty if the vote was retracted.
    pub option_ids: Vec<i64>,
}
// Divider: all content below this line will be preserved after code regen
