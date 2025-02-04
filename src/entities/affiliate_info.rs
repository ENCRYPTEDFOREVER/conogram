use serde::{Deserialize, Serialize};

use crate::entities::{chat::Chat, user::User};

/// Contains information about the affiliate that received a commission via this transaction.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#affiliateinfo)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffiliateInfo {
    /// *Optional*. The bot or the user that received an affiliate commission if it was received by a bot or a user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affiliate_user: Option<User>,

    /// *Optional*. The chat that received an affiliate commission if it was received by a chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affiliate_chat: Option<Box<Chat>>,

    /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the bot from referred users
    pub commission_per_mille: i64,

    /// Integer amount of Telegram Stars received by the affiliate from the transaction, rounded to 0; can be negative for refunds
    pub amount: i64,

    /// *Optional*. The number of 1/1000000000 shares of Telegram Stars received by the affiliate; from -999999999 to 999999999; can be negative for refunds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
