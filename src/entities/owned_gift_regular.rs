use serde::{Deserialize, Serialize};

use crate::{
    entities::{gift::Gift, message_entity::MessageEntity, user::User},
    utils::deserialize_utils::is_false,
};

/// Describes a regular gift owned by a user or a chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#ownedgiftregular)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OwnedGiftRegular {
    /// Information about the regular gift
    pub gift: Gift,

    /// *Optional*. Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,

    /// *Optional*. Sender of the gift if it is a known user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_user: Option<User>,

    /// Date the gift was sent in Unix time
    pub send_date: i64,

    /// *Optional*. Text of the message that was added to the gift
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// *Optional*. Special entities that appear in the text
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<MessageEntity>,

    /// *Optional*. True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_private: bool,

    /// *Optional*. True, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_saved: bool,

    /// *Optional*. True, if the gift can be upgraded to a unique gift; for gifts received on behalf of business accounts only
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_be_upgraded: bool,

    /// *Optional*. True, if the gift was refunded and isn't available anymore
    #[serde(default, skip_serializing_if = "is_false")]
    pub was_refunded: bool,

    /// *Optional*. Number of Telegram Stars that can be claimed by the receiver instead of the gift; omitted if the gift cannot be converted to Telegram Stars
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub convert_star_count: Option<i64>,

    /// *Optional*. Number of Telegram Stars that were paid by the sender for the ability to upgrade the gift
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prepaid_upgrade_star_count: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
