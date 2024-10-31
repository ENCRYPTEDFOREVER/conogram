use serde::{Deserialize, Serialize};

/// Describes a transaction with payment for [paid broadcasting](https://core.telegram.org/bots/api/#paid-broadcasts).
///
/// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartnertelegramapi)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartnerTelegramApi {
    /// The number of successful requests that exceeded regular limits and were therefore billed
    pub request_count: i64,
}

// Divider: all content below this line will be preserved after code regen
