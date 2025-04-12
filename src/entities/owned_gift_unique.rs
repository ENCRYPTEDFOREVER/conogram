use serde::{Deserialize, Serialize};

use crate::{
    entities::{unique_gift::UniqueGift, user::User},
    utils::deserialize_utils::is_false,
};

/// Describes a unique gift received and owned by a user or a chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#ownedgiftunique)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OwnedGiftUnique {
    /// Information about the unique gift
    pub gift: UniqueGift,

    /// *Optional*. Unique identifier of the received gift for the bot; for gifts received on behalf of business accounts only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,

    /// *Optional*. Sender of the gift if it is a known user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_user: Option<User>,

    /// Date the gift was sent in Unix time
    pub send_date: i64,

    /// *Optional*. True, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_saved: bool,

    /// *Optional*. True, if the gift can be transferred to another owner; for gifts received on behalf of business accounts only
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_be_transferred: bool,

    /// *Optional*. Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_star_count: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
