use serde::{Deserialize, Serialize};

use crate::entities::suggested_post_price::SuggestedPostPrice;

/// Contains parameters of a post that is being suggested by the bot.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#suggestedpostparameters)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuggestedPostParameters {
    /// *Optional*. Proposed price for the post. If the field is omitted, then the post is unpaid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<SuggestedPostPrice>,

    /// *Optional*. Proposed send date of the post. If specified, then the date must be between 300 second and 2678400 seconds (30 days) in the future. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user who approves it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
