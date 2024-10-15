use serde::{Deserialize, Serialize};

use crate::entities::message_entity::MessageEntity;

/// This object contains information about one answer option in a poll.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#polloption)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: String,

    /// *Optional*. Special entities that appear in the option *text*. Currently, only custom emoji entities are allowed in poll option texts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_entities: Vec<MessageEntity>,

    /// Number of users that voted for this option
    pub voter_count: i64,
}

// Divider: all content below this line will be preserved after code regen
