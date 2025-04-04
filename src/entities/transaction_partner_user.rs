use serde::{Deserialize, Serialize};

use crate::entities::{
    affiliate_info::AffiliateInfo, gift::Gift, paid_media::PaidMedia, user::User,
};

/// Describes a transaction with a user.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartneruser)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartnerUser {
    /// Information about the user
    pub user: User,

    /// *Optional*. Information about the affiliate that received a commission via this transaction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affiliate: Option<AffiliateInfo>,

    /// *Optional*. Bot-specified invoice payload
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice_payload: Option<String>,

    /// *Optional*. The duration of the paid subscription
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,

    /// *Optional*. Information about the paid media bought by the user
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paid_media: Vec<PaidMedia>,

    /// *Optional*. Bot-specified paid media payload
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_media_payload: Option<String>,

    /// *Optional*. The gift sent to the user by the bot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gift: Option<Gift>,
}

// Divider: all content below this line will be preserved after code regen
