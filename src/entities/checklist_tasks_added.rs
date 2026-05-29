use serde::{Deserialize, Serialize};

use crate::entities::{checklist_task::ChecklistTask, message::Message};

/// Describes a service message about tasks added to a checklist.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#checklisttasksadded)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChecklistTasksAdded {
    /// *Optional*. Message containing the checklist to which the tasks were added. Note that the [Message](https://core.telegram.org/bots/api/#message) object in this field will not contain the *reply\_to\_message* field even if it itself is a reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_message: Option<Box<Message>>,

    /// List of tasks added to the checklist
    pub tasks: Vec<ChecklistTask>,
}

// Divider: all content below this line will be preserved after code regen
