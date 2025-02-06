use conogram_derives::Request;
use serde::Serialize;

use crate::entities::shipping_option::ShippingOption;

/// If you sent an invoice requesting a shipping address and the parameter *is\_flexible* was specified, the Bot API will send an [Update](https://core.telegram.org/bots/api/#update) with a *shipping\_query* field to the bot. Use this method to reply to shipping queries. On success, *True* is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#answershippingquery)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct AnswerShippingQueryParams {
    /// Unique identifier for the query to be answered
    pub shipping_query_id: String,

    /// Pass *True* if delivery to the specified address is possible and *False* if there are any problems (for example, if delivery to the specified address is not possible)
    pub ok: bool,

    /// Required if *ok* is *True*. A JSON-serialized array of available shipping options.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shipping_options: Vec<ShippingOption>,

    /// Required if *ok* is *False*. Error message in human readable form that explains why it is impossible to complete the order (e.g. “Sorry, delivery to your desired address is unavailable”). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
