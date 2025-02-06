use conogram_derives::Request;
use serde::Serialize;

use crate::entities::star_transactions::StarTransactions;

/// Returns the bot's Telegram Star transactions in chronological order. On success, returns a [StarTransactions](https://core.telegram.org/bots/api/#startransactions) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getstartransactions)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = StarTransactions)]
pub struct GetStarTransactionsParams {
    /// Number of transactions to skip in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// The maximum number of transactions to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
