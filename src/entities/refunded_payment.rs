use serde::{Deserialize, Serialize};

///This object contains basic information about a refunded payment.
///API Reference: [link](https://core.telegram.org/bots/api/#refundedpayment)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RefundedPayment {
    ///Total refunded price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45`, `total_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,

    ///Bot-specified invoice payload
    pub invoice_payload: String,

    ///Telegram payment identifier
    pub telegram_payment_charge_id: String,

    ///*Optional*. Provider payment identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_payment_charge_id: Option<String>,
}
// Divider: all content below this line will be preserved after code regen
