use serde::{Deserialize, Serialize};

use crate::{
    entities::{gift::Gift, message_entity::MessageEntity},
    utils::deserialize_utils::is_false,
};

/// Describes a service message about a regular gift that was sent or received.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#giftinfo)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GiftInfo {
    /// Information about the gift
    pub gift: Gift,

    /// *Optional*. Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,

    /// *Optional*. Number of Telegram Stars that can be claimed by the receiver by converting the gift; omitted if conversion to Telegram Stars is impossible
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub convert_star_count: Option<i64>,

    /// *Optional*. Number of Telegram Stars that were prepaid by the sender for the ability to upgrade the gift
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prepaid_upgrade_star_count: Option<i64>,

    /// *Optional*. True, if the gift can be upgraded to a unique gift
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_be_upgraded: bool,

    /// *Optional*. Text of the message that was added to the gift
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// *Optional*. Special entities that appear in the text
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<MessageEntity>,

    /// *Optional*. True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_private: bool,
}

// Divider: all content below this line will be preserved after code regen
