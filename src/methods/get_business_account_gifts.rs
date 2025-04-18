use conogram_derives::Request;
use serde::Serialize;

use crate::{entities::owned_gifts::OwnedGifts, utils::deserialize_utils::is_false};

/// Returns the gifts received and owned by a managed business account. Requires the *can\_view\_gifts\_and\_stars* business bot right. Returns [OwnedGifts](https://core.telegram.org/bots/api/#ownedgifts) on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getbusinessaccountgifts)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = OwnedGifts)]
pub struct GetBusinessAccountGiftsParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// Pass True to exclude gifts that aren't saved to the account's profile page
    #[serde(skip_serializing_if = "is_false")]
    pub exclude_unsaved: bool,

    /// Pass True to exclude gifts that are saved to the account's profile page
    #[serde(skip_serializing_if = "is_false")]
    pub exclude_saved: bool,

    /// Pass True to exclude gifts that can be purchased an unlimited number of times
    #[serde(skip_serializing_if = "is_false")]
    pub exclude_unlimited: bool,

    /// Pass True to exclude gifts that can be purchased a limited number of times
    #[serde(skip_serializing_if = "is_false")]
    pub exclude_limited: bool,

    /// Pass True to exclude unique gifts
    #[serde(skip_serializing_if = "is_false")]
    pub exclude_unique: bool,

    /// Pass True to sort results by gift price instead of send date. Sorting is applied before pagination.
    #[serde(skip_serializing_if = "is_false")]
    pub sort_by_price: bool,

    /// Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,

    /// The maximum number of gifts to be returned; 1-100. Defaults to 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
