use serde::{Deserialize, Serialize};

use crate::entities::{
    transaction_partner_fragment::TransactionPartnerFragment,
    transaction_partner_other::TransactionPartnerOther,
    transaction_partner_telegram_ads::TransactionPartnerTelegramAds,
    transaction_partner_telegram_api::TransactionPartnerTelegramApi,
    transaction_partner_user::TransactionPartnerUser,
};

/// This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of
///
/// * [TransactionPartnerUser](https://core.telegram.org/bots/api/#transactionpartneruser)
/// * [TransactionPartnerFragment](https://core.telegram.org/bots/api/#transactionpartnerfragment)
/// * [TransactionPartnerTelegramAds](https://core.telegram.org/bots/api/#transactionpartnertelegramads)
/// * [TransactionPartnerTelegramApi](https://core.telegram.org/bots/api/#transactionpartnertelegramapi)
/// * [TransactionPartnerOther](https://core.telegram.org/bots/api/#transactionpartnerother)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartner)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionPartner {
    /// Describes a transaction with a user.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartneruser)
    #[serde(rename = "user")]
    User(TransactionPartnerUser),

    /// Describes a withdrawal transaction with Fragment.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartnerfragment)
    #[serde(rename = "fragment")]
    Fragment(TransactionPartnerFragment),

    /// Describes a withdrawal transaction to the Telegram Ads platform.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartnertelegramads)
    #[serde(rename = "telegram_ads")]
    TelegramAds(TransactionPartnerTelegramAds),

    /// Describes a transaction with payment for [paid broadcasting](https://core.telegram.org/bots/api/#paid-broadcasts).
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartnertelegramapi)
    #[serde(rename = "telegram_api")]
    TelegramApi(TransactionPartnerTelegramApi),

    /// Describes a transaction with an unknown source or recipient.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartnerother)
    #[serde(rename = "other")]
    Other(TransactionPartnerOther),
}

impl Default for TransactionPartner {
    fn default() -> Self {
        Self::User(TransactionPartnerUser::default())
    }
}

impl From<TransactionPartnerUser> for TransactionPartner {
    fn from(value: TransactionPartnerUser) -> Self {
        Self::User(value)
    }
}

impl From<TransactionPartnerFragment> for TransactionPartner {
    fn from(value: TransactionPartnerFragment) -> Self {
        Self::Fragment(value)
    }
}

impl From<TransactionPartnerTelegramAds> for TransactionPartner {
    fn from(value: TransactionPartnerTelegramAds) -> Self {
        Self::TelegramAds(value)
    }
}

impl From<TransactionPartnerTelegramApi> for TransactionPartner {
    fn from(value: TransactionPartnerTelegramApi) -> Self {
        Self::TelegramApi(value)
    }
}

impl From<TransactionPartnerOther> for TransactionPartner {
    fn from(value: TransactionPartnerOther) -> Self {
        Self::Other(value)
    }
}

// Divider: all content below this line will be preserved after code regen
