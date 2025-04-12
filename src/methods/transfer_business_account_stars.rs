use conogram_derives::Request;
use serde::Serialize;

/// Transfers Telegram Stars from the business account balance to the bot's balance. Requires the *can\_transfer\_stars* business bot right. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#transferbusinessaccountstars)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct TransferBusinessAccountStarsParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// Number of Telegram Stars to transfer; 1-10000
    pub star_count: i64,
}

// Divider: all content below this line will be preserved after code regen
