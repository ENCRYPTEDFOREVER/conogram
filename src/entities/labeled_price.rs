use serde::{Deserialize, Serialize};

/// This object represents a portion of the price for goods or services.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#labeledprice)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabeledPrice {
    /// Portion label
    pub label: String,

    /// Price of the product in the *smallest units* of the [currency](https://core.telegram.org/bots/payments#supported-currencies) (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub amount: i64,
}

// Divider: all content below this line will be preserved after code regen
