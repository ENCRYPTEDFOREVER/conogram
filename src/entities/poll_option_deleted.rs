use serde::{Deserialize, Serialize};

use crate::entities::{
    maybe_inaccessible_message::MaybeInaccessibleMessage, message_entity::MessageEntity,
};

/// Describes a service message about an option deleted from a poll.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#polloptiondeleted)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PollOptionDeleted {
    /// *Optional*. Message containing the poll from which the option was deleted, if known. Note that the [Message](https://core.telegram.org/bots/api/#message) object in this field will not contain the *reply\_to\_message* field even if it itself is a reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll_message: Option<Box<MaybeInaccessibleMessage>>,

    /// Unique identifier of the deleted option
    pub option_persistent_id: String,

    /// Option text
    pub option_text: String,

    /// *Optional*. Special entities that appear in the *option\_text*
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub option_text_entities: Vec<MessageEntity>,
}

// Divider: all content below this line will be preserved after code regen
