use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// Describes a topic of a direct messages chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#directmessagestopic)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectMessagesTopic {
    /// Unique identifier of the topic. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub topic_id: i64,

    /// *Optional*. Information about the user that created the topic. Currently, it is always present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

// Divider: all content below this line will be preserved after code regen
