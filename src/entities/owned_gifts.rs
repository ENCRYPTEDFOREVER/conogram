use serde::{Deserialize, Serialize};

use crate::entities::owned_gift::OwnedGift;

/// Contains the list of gifts received and owned by a user or a chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#ownedgifts)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OwnedGifts {
    /// The total number of gifts owned by the user or the chat
    pub total_count: i64,

    /// The list of gifts
    pub gifts: Vec<OwnedGift>,

    /// *Optional*. Offset for the next request. If empty, then there are no more results
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
