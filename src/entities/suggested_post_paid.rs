use serde::{Deserialize, Serialize};

use crate::entities::{message::Message, star_amount::StarAmount};

/// Describes a service message about a successful payment for a suggested post.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#suggestedpostpaid)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostPaid {
    /// *Optional*. Message containing the suggested post. Note that the [Message](https://core.telegram.org/bots/api/#message) object in this field will not contain the *reply\_to\_message* field even if it itself is a reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<Message>>,

    /// Currency in which the payment was made. Currently, one of “XTR” for Telegram Stars or “TON” for toncoins.
    pub currency: SuggestedPostPaidCurrency,

    /// *Optional*. The amount of the currency that was received by the channel in nanotoncoins; for payments in toncoins only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// *Optional*. The amount of Telegram Stars that was received by the channel; for payments in Telegram Stars only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub star_amount: Option<StarAmount>,
}

/// Currency in which the payment was made. Currently, one of “XTR” for Telegram Stars or “TON” for toncoins.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuggestedPostPaidCurrency {
    /// `XTR`
    #[default]
    #[serde(rename = "XTR")]
    Xtr,

    /// `TON`
    #[serde(rename = "TON")]
    Ton,
}

// Divider: all content below this line will be preserved after code regen
