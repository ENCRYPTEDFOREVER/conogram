use serde::{Deserialize, Serialize};

use crate::entities::{owned_gift_regular::OwnedGiftRegular, owned_gift_unique::OwnedGiftUnique};

/// This object describes a gift received and owned by a user or a chat. Currently, it can be one of
///
/// * [OwnedGiftRegular](https://core.telegram.org/bots/api/#ownedgiftregular)
/// * [OwnedGiftUnique](https://core.telegram.org/bots/api/#ownedgiftunique)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#ownedgift)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OwnedGift {
    /// Describes a regular gift owned by a user or a chat.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#ownedgiftregular)
    #[serde(rename = "regular")]
    Regular(OwnedGiftRegular),

    /// Describes a unique gift received and owned by a user or a chat.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#ownedgiftunique)
    #[serde(rename = "unique")]
    Unique(OwnedGiftUnique),
}

impl Default for OwnedGift {
    fn default() -> Self {
        Self::Regular(OwnedGiftRegular::default())
    }
}

impl From<OwnedGiftRegular> for OwnedGift {
    fn from(value: OwnedGiftRegular) -> Self {
        Self::Regular(value)
    }
}

impl From<OwnedGiftUnique> for OwnedGift {
    fn from(value: OwnedGiftUnique) -> Self {
        Self::Unique(value)
    }
}

// Divider: all content below this line will be preserved after code regen
