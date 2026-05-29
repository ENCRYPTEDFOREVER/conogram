use serde::{Deserialize, Serialize};

use crate::entities::message::Message;

/// Describes a service message about the rejection of a suggested post.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#suggestedpostdeclined)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostDeclined {
    /// *Optional*. Message containing the suggested post. Note that the [Message](https://core.telegram.org/bots/api/#message) object in this field will not contain the *reply\_to\_message* field even if it itself is a reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,

    /// *Optional*. Comment with which the post was declined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
