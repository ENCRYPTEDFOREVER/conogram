use serde::{Deserialize, Serialize};

use crate::entities::{
    affiliate_info::AffiliateInfo, gift::Gift, paid_media::PaidMedia, user::User,
};

/// Describes a transaction with a user.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartneruser)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartnerUser {
    /// Type of the transaction, currently one of “invoice\_payment” for payments via invoices, “paid\_media\_payment” for payments for paid media, “gift\_purchase” for gifts sent by the bot, “premium\_purchase” for Telegram Premium subscriptions gifted by the bot, “business\_account\_transfer” for direct transfers from managed business accounts
    pub transaction_type: TransactionType,

    /// Information about the user
    pub user: User,

    /// *Optional*. Information about the affiliate that received a commission via this transaction. Can be available only for “invoice\_payment” and “paid\_media\_payment” transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affiliate: Option<AffiliateInfo>,

    /// *Optional*. Bot-specified invoice payload. Can be available only for “invoice\_payment” transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice_payload: Option<String>,

    /// *Optional*. The duration of the paid subscription. Can be available only for “invoice\_payment” transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,

    /// *Optional*. Information about the paid media bought by the user; for “paid\_media\_payment” transactions only
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paid_media: Vec<PaidMedia>,

    /// *Optional*. Bot-specified paid media payload. Can be available only for “paid\_media\_payment” transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_media_payload: Option<String>,

    /// *Optional*. The gift sent to the user by the bot; for “gift\_purchase” transactions only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gift: Option<Gift>,

    /// *Optional*. Number of months the gifted Telegram Premium subscription will be active for; for “premium\_purchase” transactions only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_subscription_duration: Option<i64>,
}

/// Type of the transaction, currently one of “invoice\_payment” for payments via invoices, “paid\_media\_payment” for payments for paid media, “gift\_purchase” for gifts sent by the bot, “premium\_purchase” for Telegram Premium subscriptions gifted by the bot, “business\_account\_transfer” for direct transfers from managed business accounts
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    /// `invoice_payment`
    #[default]
    #[serde(rename = "invoice_payment")]
    InvoicePayment,

    /// `paid_media_payment`
    #[serde(rename = "paid_media_payment")]
    PaidMediaPayment,

    /// `gift_purchase`
    #[serde(rename = "gift_purchase")]
    GiftPurchase,

    /// `premium_purchase`
    #[serde(rename = "premium_purchase")]
    PremiumPurchase,

    /// `business_account_transfer`
    #[serde(rename = "business_account_transfer")]
    BusinessAccountTransfer,
}

// Divider: all content below this line will be preserved after code regen
