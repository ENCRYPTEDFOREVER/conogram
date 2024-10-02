use crate::entities::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// This object contains information about one answer option in a poll to be sent.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputpolloption)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct InputPollOption {
    /// Option text, 1-100 characters
    pub text: String,

    /// *Optional*. Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Currently, only custom emoji entities are allowed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,

    /// *Optional*. A JSON-serialized list of special entities that appear in the poll option text. It can be specified instead of *text\_parse\_mode*
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_entities: Vec<MessageEntity>,
}

// Divider: all content below this line will be preserved after code regen
