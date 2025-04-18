use conogram_derives::Request;
use serde::Serialize;

use crate::utils::deserialize_utils::is_false;

/// Upgrades a given regular gift to a unique gift. Requires the *can\_transfer\_and\_upgrade\_gifts* business bot right. Additionally requires the *can\_transfer\_stars* business bot right if the upgrade is paid. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#upgradegift)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct UpgradeGiftParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// Unique identifier of the regular gift that should be upgraded to a unique one
    pub owned_gift_id: String,

    /// Pass True to keep the original gift text, sender and receiver in the upgraded gift
    #[serde(skip_serializing_if = "is_false")]
    pub keep_original_details: bool,

    /// The amount of Telegram Stars that will be paid for the upgrade from the business account balance. If `gift.prepaid_upgrade_star_count > 0`, then pass 0, otherwise, the *can\_transfer\_stars* business bot right is required and `gift.upgrade_star_count` must be passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_count: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
