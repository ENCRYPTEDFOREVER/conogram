use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// Describes the affiliate program that issued the affiliate commission received via this transaction.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartneraffiliateprogram)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionPartnerAffiliateProgram {
    /// *Optional*. Information about the bot that sponsored the affiliate program
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sponsor_user: Option<User>,

    /// The number of Telegram Stars received by the bot for each 1000 Telegram Stars received by the affiliate program sponsor from referred users
    pub commission_per_mille: i64,
}

// Divider: all content below this line will be preserved after code regen
