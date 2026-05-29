use serde::{Deserialize, Serialize};

use crate::entities::{chat::Chat, message_entity::MessageEntity, user::User};

/// Describes a task in a checklist.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#checklisttask)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChecklistTask {
    /// Unique identifier of the task
    pub id: i64,

    /// Text of the task
    pub text: String,

    /// *Optional*. Special entities that appear in the task text
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_entities: Vec<MessageEntity>,

    /// *Optional*. User that completed the task; omitted if the task wasn't completed by a user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_by_user: Option<User>,

    /// *Optional*. Chat that completed the task; omitted if the task wasn't completed by a chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_by_chat: Option<Box<Chat>>,

    /// *Optional*. Point in time (Unix timestamp) when the task was completed; 0 if the task wasn't completed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
