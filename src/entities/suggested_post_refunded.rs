use serde::{Deserialize, Serialize};

use crate::entities::message::Message;

/// Describes a service message about a payment refund for a suggested post.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#suggestedpostrefunded)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostRefunded {
    /// *Optional*. Message containing the suggested post. Note that the [Message](https://core.telegram.org/bots/api/#message) object in this field will not contain the *reply\_to\_message* field even if it itself is a reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,

    /// Reason for the refund. Currently, one of “post\_deleted” if the post was deleted within 24 hours of being posted or removed from scheduled messages without being posted, or “payment\_refunded” if the payer refunded their payment.
    pub reason: SuggestedPostRefundedReason,
}

/// Reason for the refund. Currently, one of “post\_deleted” if the post was deleted within 24 hours of being posted or removed from scheduled messages without being posted, or “payment\_refunded” if the payer refunded their payment.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuggestedPostRefundedReason {
    /// `post_deleted`
    #[default]
    #[serde(rename = "post_deleted")]
    PostDeleted,

    /// `payment_refunded`
    #[serde(rename = "payment_refunded")]
    PaymentRefunded,
}

// Divider: all content below this line will be preserved after code regen
