use conogram_derives::Request;
use serde::Serialize;

use crate::{entities::owned_gifts::OwnedGifts, utils::deserialize_utils::is_false};

/// Returns the gifts owned and hosted by a user. Returns [OwnedGifts](https://core.telegram.org/bots/api/#ownedgifts) on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getusergifts)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = OwnedGifts)]
pub struct GetUserGiftsParams {
    /// Unique identifier of the user
    pub user_id: i64,

    /// Pass *True* to exclude gifts that can be purchased an unlimited number of times
    #[serde(skip_serializing_if = "is_false")]
    pub exclude_unlimited: bool,

    /// Pass *True* to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
    #[serde(skip_serializing_if = "is_false")]
    pub exclude_limited_upgradable: bool,

    /// Pass *True* to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
    #[serde(skip_serializing_if = "is_false")]
    pub exclude_limited_non_upgradable: bool,

    /// Pass *True* to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[serde(skip_serializing_if = "is_false")]
    pub exclude_from_blockchain: bool,

    /// Pass *True* to exclude unique gifts
    #[serde(skip_serializing_if = "is_false")]
    pub exclude_unique: bool,

    /// Pass *True* to sort results by gift price instead of send date. Sorting is applied before pagination.
    #[serde(skip_serializing_if = "is_false")]
    pub sort_by_price: bool,

    /// Offset of the first entry to return as received from the previous request; use an empty string to get the first chunk of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,

    /// The maximum number of gifts to be returned; 1-100. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
