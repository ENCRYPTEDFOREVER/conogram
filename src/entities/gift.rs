use serde::{Deserialize, Serialize};

use crate::entities::sticker::Sticker;

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

    /// *Optional*. The total number of the gifts of this type that can be sent; for limited gifts only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,

    /// *Optional*. The number of remaining gifts of this type that can be sent; for limited gifts only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
