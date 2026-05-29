use serde::{Deserialize, Serialize};

use crate::entities::{message::Message, suggested_post_price::SuggestedPostPrice};

/// Describes a service message about the failed approval of a suggested post. Currently, only caused by insufficient user funds at the time of approval.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#suggestedpostapprovalfailed)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostApprovalFailed {
    /// *Optional*. Message containing the suggested post whose approval has failed. Note that the [Message](https://core.telegram.org/bots/api/#message) object in this field will not contain the *reply\_to\_message* field even if it itself is a reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,

    /// Expected price of the post
    pub price: SuggestedPostPrice,
}

// Divider: all content below this line will be preserved after code regen
