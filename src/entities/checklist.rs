use serde::{Deserialize, Serialize};

use crate::{
    entities::{checklist_task::ChecklistTask, message_entity::MessageEntity},
    utils::deserialize_utils::is_false,
};

/// Describes a checklist.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#checklist)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checklist {
    /// Title of the checklist
    pub title: String,

    /// *Optional*. Special entities that appear in the checklist title
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub title_entities: Vec<MessageEntity>,

    /// List of tasks in the checklist
    pub tasks: Vec<ChecklistTask>,

    /// *Optional*. *True*, if users other than the creator of the list can add tasks to the list
    #[serde(default, skip_serializing_if = "is_false")]
    pub others_can_add_tasks: bool,

    /// *Optional*. *True*, if users other than the creator of the list can mark tasks as done or not done
    #[serde(default, skip_serializing_if = "is_false")]
    pub others_can_mark_tasks_as_done: bool,
}

// Divider: all content below this line will be preserved after code regen
