use serde::Serialize;

use crate::{entities::labeled_price::LabeledPrice, utils::deserialize_utils::is_false};

/// Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of an invoice message to be sent as the result of an inline query.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputinvoicemessagecontent)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InputInvoiceMessageContent {
    /// Product name, 1-32 characters
    pub title: String,

    /// Product description, 1-255 characters
    pub description: String,

    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    pub payload: String,

    /// *Optional*. Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,

    /// Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/payments#supported-currencies). Pass “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90).
    pub currency: String,

    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in [Telegram Stars](https://t.me/BotNews/90).
    pub prices: Vec<LabeledPrice>,

    /// *Optional*. The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,

    /// *Optional*. A JSON-serialized array of suggested amounts of tip in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max\_tip\_amount*.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggested_tip_amounts: Vec<i64>,

    /// *Optional*. A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,

    /// *Optional*. URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,

    /// *Optional*. Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,

    /// *Optional*. Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,

    /// *Optional*. Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,

    /// *Optional*. Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub need_name: bool,

    /// *Optional*. Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub need_phone_number: bool,

    /// *Optional*. Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub need_email: bool,

    /// *Optional*. Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub need_shipping_address: bool,

    /// *Optional*. Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub send_phone_number_to_provider: bool,

    /// *Optional*. Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub send_email_to_provider: bool,

    /// *Optional*. Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub is_flexible: bool,
}

// Divider: all content below this line will be preserved after code regen
