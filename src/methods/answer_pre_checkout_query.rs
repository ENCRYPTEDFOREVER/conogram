use conogram_derives::Request;
use serde::Serialize;

/// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an [Update](https://core.telegram.org/bots/api/#update) with the field *pre\_checkout\_query*. Use this method to respond to such pre-checkout queries. On success, *True* is returned. **Note:** The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#answerprecheckoutquery)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct AnswerPreCheckoutQueryParams {
    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: String,

    /// Specify *True* if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use *False* if there are any problems.
    pub ok: bool,

    /// Required if *ok* is *False*. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!"). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
