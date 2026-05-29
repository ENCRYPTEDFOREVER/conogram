use serde::{Deserialize, Serialize};

use crate::entities::message::Message;

/// Describes a service message about checklist tasks marked as done or not done.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#checklisttasksdone)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChecklistTasksDone {
    /// *Optional*. Message containing the checklist whose tasks were marked as done or not done. Note that the [Message](https://core.telegram.org/bots/api/#message) object in this field will not contain the *reply\_to\_message* field even if it itself is a reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_message: Option<Box<Message>>,

    /// *Optional*. Identifiers of the tasks that were marked as done
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub marked_as_done_task_ids: Vec<i64>,

    /// *Optional*. Identifiers of the tasks that were marked as not done
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub marked_as_not_done_task_ids: Vec<i64>,
}

// Divider: all content below this line will be preserved after code regen
