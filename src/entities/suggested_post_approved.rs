use serde::{Deserialize, Serialize};

use crate::entities::{message::Message, suggested_post_price::SuggestedPostPrice};

/// Describes a service message about the approval of a suggested post.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#suggestedpostapproved)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostApproved {
    /// *Optional*. Message containing the suggested post. Note that the [Message](https://core.telegram.org/bots/api/#message) object in this field will not contain the *reply\_to\_message* field even if it itself is a reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,

    /// *Optional*. Amount paid for the post
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<SuggestedPostPrice>,

    /// Date when the post will be published
    pub send_date: i64,
}

// Divider: all content below this line will be preserved after code regen
