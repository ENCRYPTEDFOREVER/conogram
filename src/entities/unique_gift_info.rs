use serde::{Deserialize, Serialize};

use crate::entities::{suggested_post_paid::SuggestedPostPaidCurrency, unique_gift::UniqueGift};

/// Describes a service message about a unique gift that was sent or received.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#uniquegiftinfo)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UniqueGiftInfo {
    /// Information about the gift
    pub gift: UniqueGift,

    /// Origin of the gift. Currently, either “upgrade” for gifts upgraded from regular gifts, “transfer” for gifts transferred from other users or channels, “resale” for gifts bought from other users, “gifted\_upgrade” for upgrades purchased after the gift was sent, or “offer” for gifts bought or sold through gift purchase offers.
    pub origin: UniqueGiftInfoOrigin,

    /// *Optional*. For gifts bought from other users, the currency in which the payment for the gift was done. Currently, one of “XTR” for Telegram Stars or “TON” for toncoins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_resale_currency: Option<SuggestedPostPaidCurrency>,

    /// *Optional*. For gifts bought from other users, the price paid for the gift in either Telegram Stars or nanotoncoins
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_resale_amount: Option<i64>,

    /// *Optional*. Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,

    /// *Optional*. Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_star_count: Option<i64>,

    /// *Optional*. Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_transfer_date: Option<i64>,
}

/// Origin of the gift. Currently, either “upgrade” for gifts upgraded from regular gifts, “transfer” for gifts transferred from other users or channels, “resale” for gifts bought from other users, “gifted\_upgrade” for upgrades purchased after the gift was sent, or “offer” for gifts bought or sold through gift purchase offers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum UniqueGiftInfoOrigin {
    /// `upgrade`
    #[default]
    #[serde(rename = "upgrade")]
    Upgrade,

    /// `transfer`
    #[serde(rename = "transfer")]
    Transfer,

    /// `resale`
    #[serde(rename = "resale")]
    Resale,

    /// `gifted_upgrade`
    #[serde(rename = "gifted_upgrade")]
    GiftedUpgrade,

    /// `offer`
    #[serde(rename = "offer")]
    Offer,
}

// Divider: all content below this line will be preserved after code regen
