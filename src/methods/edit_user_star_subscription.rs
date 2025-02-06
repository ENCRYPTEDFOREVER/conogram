use conogram_derives::Request;
use serde::Serialize;

/// Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#edituserstarsubscription)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct EditUserStarSubscriptionParams {
    /// Identifier of the user whose subscription will be edited
    pub user_id: i64,

    /// Telegram payment identifier for the subscription
    pub telegram_payment_charge_id: String,

    /// Pass *True* to cancel extension of the user subscription; the subscription must be active up to the end of the current subscription period. Pass *False* to allow the user to re-enable a subscription that was previously canceled by the bot.
    pub is_canceled: bool,
}

// Divider: all content below this line will be preserved after code regen
