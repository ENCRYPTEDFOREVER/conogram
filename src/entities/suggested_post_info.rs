use serde::{Deserialize, Serialize};

use crate::entities::suggested_post_price::SuggestedPostPrice;

/// Contains information about a suggested post.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#suggestedpostinfo)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuggestedPostInfo {
    /// State of the suggested post. Currently, it can be one of “pending”, “approved”, “declined”.
    pub state: SuggestedPostInfoState,

    /// *Optional*. Proposed price of the post. If the field is omitted, then the post is unpaid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<SuggestedPostPrice>,

    /// *Optional*. Proposed send date of the post. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user or administrator who approves it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}

/// State of the suggested post. Currently, it can be one of “pending”, “approved”, “declined”.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuggestedPostInfoState {
    /// `pending`
    #[default]
    #[serde(rename = "pending")]
    Pending,

    /// `approved`
    #[serde(rename = "approved")]
    Approved,

    /// `declined`
    #[serde(rename = "declined")]
    Declined,
}

// Divider: all content below this line will be preserved after code regen
