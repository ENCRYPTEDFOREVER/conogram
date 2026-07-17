use serde::Serialize;

use crate::entities::message_entity::MessageEntity;

/// Describes a task to add to a checklist.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputchecklisttask)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InputChecklistTask {
    /// Unique identifier of the task; must be positive and unique among all task identifiers currently present in the checklist
    pub id: i64,

    /// Text of the task; 1-100 characters after entities parsing
    pub text: String,

    /// *Optional*. Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// *Optional*. List of special entities that appear in the text, which can be specified instead of parse\_mode. Currently, only *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, *custom\_emoji*, and *date\_time* entities are allowed.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub text_entities: Vec<MessageEntity>,
}

// Divider: all content below this line will be preserved after code regen
