use serde::{Deserialize, Serialize};

use crate::entities::{
    transaction_partner_affiliate_program::TransactionPartnerAffiliateProgram,
    transaction_partner_chat::TransactionPartnerChat,
    transaction_partner_fragment::TransactionPartnerFragment,
    transaction_partner_other::TransactionPartnerOther,
    transaction_partner_telegram_ads::TransactionPartnerTelegramAds,
    transaction_partner_telegram_api::TransactionPartnerTelegramApi,
    transaction_partner_user::TransactionPartnerUser,
};

/// This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of
///
/// * [TransactionPartnerUser](https://core.telegram.org/bots/api/#transactionpartneruser)
/// * [TransactionPartnerChat](https://core.telegram.org/bots/api/#transactionpartnerchat)
/// * [TransactionPartnerAffiliateProgram](https://core.telegram.org/bots/api/#transactionpartneraffiliateprogram)
/// * [TransactionPartnerFragment](https://core.telegram.org/bots/api/#transactionpartnerfragment)
/// * [TransactionPartnerTelegramAds](https://core.telegram.org/bots/api/#transactionpartnertelegramads)
/// * [TransactionPartnerTelegramApi](https://core.telegram.org/bots/api/#transactionpartnertelegramapi)
/// * [TransactionPartnerOther](https://core.telegram.org/bots/api/#transactionpartnerother)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartner)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionPartner {
    /// Describes a transaction with a user.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartneruser)
    User(TransactionPartnerUser),

    /// Describes a transaction with a chat.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartnerchat)
    Chat(TransactionPartnerChat),

    /// Describes the affiliate program that issued the affiliate commission received via this transaction.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartneraffiliateprogram)
    AffiliateProgram(TransactionPartnerAffiliateProgram),

    /// Describes a withdrawal transaction with Fragment.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartnerfragment)
    Fragment(TransactionPartnerFragment),

    /// Describes a withdrawal transaction to the Telegram Ads platform.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartnertelegramads)
    TelegramAds(TransactionPartnerTelegramAds),

    /// Describes a transaction with payment for [paid broadcasting](https://core.telegram.org/bots/api/#paid-broadcasts).
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartnertelegramapi)
    TelegramApi(TransactionPartnerTelegramApi),

    /// Describes a transaction with an unknown source or recipient.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartnerother)
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

impl From<TransactionPartnerChat> for TransactionPartner {
    fn from(value: TransactionPartnerChat) -> Self {
        Self::Chat(value)
    }
}

impl From<TransactionPartnerAffiliateProgram> for TransactionPartner {
    fn from(value: TransactionPartnerAffiliateProgram) -> Self {
        Self::AffiliateProgram(value)
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
