use conogram_derives::Request;
use serde::Serialize;

use crate::entities::star_amount::StarAmount;

/// Returns the amount of Telegram Stars owned by a managed business account. Requires the *can\_view\_gifts\_and\_stars* business bot right. Returns [StarAmount](https://core.telegram.org/bots/api/#staramount) on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getbusinessaccountstarbalance)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = StarAmount)]
pub struct GetBusinessAccountStarBalanceParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
}

// Divider: all content below this line will be preserved after code regen
