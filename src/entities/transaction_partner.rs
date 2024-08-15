use crate::entities::transaction_partner_fragment::TransactionPartnerFragment;
use crate::entities::transaction_partner_other::TransactionPartnerOther;
use crate::entities::transaction_partner_telegram_ads::TransactionPartnerTelegramAds;
use crate::entities::transaction_partner_user::TransactionPartnerUser;
use serde::{Deserialize, Serialize};

///This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of
///
///* [TransactionPartnerUser](https://core.telegram.org/bots/api/#transactionpartneruser)
///* [TransactionPartnerFragment](https://core.telegram.org/bots/api/#transactionpartnerfragment)
///* [TransactionPartnerTelegramAds](https://core.telegram.org/bots/api/#transactionpartnertelegramads)
///* [TransactionPartnerOther](https://core.telegram.org/bots/api/#transactionpartnerother)
///
///API Reference: [link](https://core.telegram.org/bots/api/#transactionpartner)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionPartner {
    #[serde(rename = "user")]
    User(TransactionPartnerUser),
    #[serde(rename = "fragment")]
    Fragment(TransactionPartnerFragment),
    #[serde(rename = "telegram_ads")]
    TelegramAds(TransactionPartnerTelegramAds),
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

impl From<TransactionPartnerOther> for TransactionPartner {
    fn from(value: TransactionPartnerOther) -> Self {
        Self::Other(value)
    }
}
// Divider: all content below this line will be preserved after code regen
