use serde::Serialize;

use crate::{
    entities::{input_checklist_task::InputChecklistTask, message_entity::MessageEntity},
    utils::deserialize_utils::is_false,
};

/// Describes a checklist to create.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputchecklist)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InputChecklist {
    /// Title of the checklist; 1-255 characters after entities parsing
    pub title: String,

    /// *Optional*. Mode for parsing entities in the title. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// *Optional*. List of special entities that appear in the title, which can be specified instead of parse\_mode. Currently, only *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, *custom\_emoji*, and *date\_time* entities are allowed.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub title_entities: Vec<MessageEntity>,

    /// List of 1-30 tasks in the checklist
    pub tasks: Vec<InputChecklistTask>,

    /// *Optional*. Pass *True* if other users can add tasks to the checklist
    #[serde(skip_serializing_if = "is_false")]
    pub others_can_add_tasks: bool,

    /// *Optional*. Pass *True* if other users can mark tasks as done or not done in the checklist
    #[serde(skip_serializing_if = "is_false")]
    pub others_can_mark_tasks_as_done: bool,
}

// Divider: all content below this line will be preserved after code regen
