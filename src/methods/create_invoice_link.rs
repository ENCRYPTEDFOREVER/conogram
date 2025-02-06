use conogram_derives::Request;
use serde::Serialize;

use crate::{entities::labeled_price::LabeledPrice, utils::deserialize_utils::is_false};

/// Use this method to create a link for an invoice. Returns the created invoice link as *String* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#createinvoicelink)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = String)]
pub struct CreateInvoiceLinkParams {
    /// Unique identifier of the business connection on behalf of which the link will be created. For payments in [Telegram Stars](https://t.me/BotNews/90) only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// Product name, 1-32 characters
    pub title: String,

    /// Product description, 1-255 characters
    pub description: String,

    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    pub payload: String,

    /// Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,

    /// Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/payments#supported-currencies). Pass “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90).
    pub currency: String,

    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in [Telegram Stars](https://t.me/BotNews/90).
    pub prices: Vec<LabeledPrice>,

    /// The number of seconds the subscription will be active for before the next payment. The currency must be set to “XTR” (Telegram Stars) if the parameter is used. Currently, it must always be 2592000 (30 days) if specified. Any number of subscriptions can be active for a given bot at the same time, including multiple concurrent subscriptions from the same user. Subscription price must no exceed 2500 Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,

    /// The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,

    /// A JSON-serialized array of suggested amounts of tips in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max\_tip\_amount*.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggested_tip_amounts: Vec<i64>,

    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,

    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,

    /// Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,

    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,

    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,

    /// Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub need_name: bool,

    /// Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub need_phone_number: bool,

    /// Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub need_email: bool,

    /// Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub need_shipping_address: bool,

    /// Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub send_phone_number_to_provider: bool,

    /// Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub send_email_to_provider: bool,

    /// Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[serde(skip_serializing_if = "is_false")]
    pub is_flexible: bool,
}

// Divider: all content below this line will be preserved after code regen
