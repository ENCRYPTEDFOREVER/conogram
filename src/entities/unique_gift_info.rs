use serde::{Deserialize, Serialize};

use crate::entities::unique_gift::UniqueGift;

/// Describes a service message about a unique gift that was sent or received.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#uniquegiftinfo)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UniqueGiftInfo {
    /// Information about the gift
    pub gift: UniqueGift,

    /// Origin of the gift. Currently, either “upgrade” or “transfer”
    pub origin: UniqueGiftInfoOrigin,

    /// *Optional*. Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,

    /// *Optional*. Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_star_count: Option<i64>,
}

/// Origin of the gift. Currently, either “upgrade” or “transfer”
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum UniqueGiftInfoOrigin {
    /// `upgrade`
    #[default]
    #[serde(rename = "upgrade")]
    Upgrade,

    /// `transfer`
    #[serde(rename = "transfer")]
    Transfer,
}

// Divider: all content below this line will be preserved after code regen
