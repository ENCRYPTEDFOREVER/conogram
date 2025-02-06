use conogram_derives::Request;
use serde::Serialize;

/// Refunds a successful payment in [Telegram Stars](https://t.me/BotNews/90). Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#refundstarpayment)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct RefundStarPaymentParams {
    /// Identifier of the user whose payment will be refunded
    pub user_id: i64,

    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
}

// Divider: all content below this line will be preserved after code regen
