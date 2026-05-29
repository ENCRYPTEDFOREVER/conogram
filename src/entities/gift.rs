use serde::{Deserialize, Serialize};

use crate::{
    entities::{chat::Chat, gift_background::GiftBackground, sticker::Sticker},
    utils::deserialize_utils::is_false,
};

/// This object represents a gift that can be sent by the bot.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#gift)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Gift {
    /// Unique identifier of the gift
    pub id: String,

    /// The sticker that represents the gift
    pub sticker: Sticker,

    /// The number of Telegram Stars that must be paid to send the sticker
    pub star_count: i64,

    /// *Optional*. The number of Telegram Stars that must be paid to upgrade the gift to a unique one
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upgrade_star_count: Option<i64>,

    /// *Optional*. *True*, if the gift can only be purchased by Telegram Premium subscribers
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_premium: bool,

    /// *Optional*. *True*, if the gift can be used (after being upgraded) to customize a user's appearance
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_colors: bool,

    /// *Optional*. The total number of gifts of this type that can be sent by all users; for limited gifts only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,

    /// *Optional*. The number of remaining gifts of this type that can be sent by all users; for limited gifts only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<i64>,

    /// *Optional*. The total number of gifts of this type that can be sent by the bot; for limited gifts only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_total_count: Option<i64>,

    /// *Optional*. The number of remaining gifts of this type that can be sent by the bot; for limited gifts only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_remaining_count: Option<i64>,

    /// *Optional*. Background of the gift
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<GiftBackground>,

    /// *Optional*. The total number of different unique gifts that can be obtained by upgrading the gift
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_gift_variant_count: Option<i64>,

    /// *Optional*. Information about the chat that published the gift
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher_chat: Option<Box<Chat>>,
}

// Divider: all content below this line will be preserved after code regen
