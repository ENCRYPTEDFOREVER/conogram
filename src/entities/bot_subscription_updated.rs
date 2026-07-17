use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// This object contains information about changes to a user payment subscription toward the current bot.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#botsubscriptionupdated)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotSubscriptionUpdated {
    /// User who subscribed for payments toward the bot
    pub user: User,

    /// Bot-specified invoice payload
    pub invoice_payload: String,

    /// The new state of the subscription. Currently, it can be one of “canceled” if the user canceled the subscription, “active” if the user re-enabled a previously canceled subscription, or “failed” if payment for the subscription failed.
    pub state: BotSubscriptionUpdatedState,
}

/// The new state of the subscription. Currently, it can be one of “canceled” if the user canceled the subscription, “active” if the user re-enabled a previously canceled subscription, or “failed” if payment for the subscription failed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BotSubscriptionUpdatedState {
    /// `canceled`
    #[default]
    #[serde(rename = "canceled")]
    Canceled,

    /// `active`
    #[serde(rename = "active")]
    Active,

    /// `failed`
    #[serde(rename = "failed")]
    Failed,
}

// Divider: all content below this line will be preserved after code regen
