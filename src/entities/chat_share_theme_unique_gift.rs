use serde::{Deserialize, Serialize};

use crate::entities::unique_gift::UniqueGift;

/// The theme based on a unique gift owned by the user or the chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatsharethemeuniquegift)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "unique_gift", tag = "type")]
pub struct ChatShareThemeUniqueGift {
    /// The gift
    pub gift: UniqueGift,
}

// Divider: all content below this line will be preserved after code regen
