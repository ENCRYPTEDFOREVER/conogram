use serde::{Deserialize, Serialize};

use crate::{entities::order_info::OrderInfo, utils::deserialize_utils::is_false};

/// This object contains basic information about a successful payment.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#successfulpayment)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 [currency](https://core.telegram.org/bots/payments#supported-currencies) code, or “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90)
    pub currency: String,

    /// Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,

    /// Bot-specified invoice payload
    pub invoice_payload: String,

    /// *Optional*. Expiration date of the subscription, in Unix time; for recurring payments only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_expiration_date: Option<i64>,

    /// *Optional*. True, if the payment is a recurring payment for a subscription
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_recurring: bool,

    /// *Optional*. True, if the payment is the first payment for a subscription
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_first_recurring: bool,

    /// *Optional*. Identifier of the shipping option chosen by the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,

    /// *Optional*. Order information provided by the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,

    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,

    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}

// Divider: all content below this line will be preserved after code regen
