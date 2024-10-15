use serde::{Deserialize, Serialize};

use crate::{entities::user::User, utils::deserialize_utils::is_false};

/// The boost was obtained by the creation of a Telegram Premium or a Telegram Star giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription for Telegram Premium giveaways and *prize\_star\_count* / 500 times for one year for Telegram Star giveaways.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatboostsourcegiveaway)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatBoostSourceGiveaway {
    /// Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet.
    pub giveaway_message_id: i64,

    /// *Optional*. User that won the prize in the giveaway if any; for Telegram Premium giveaways only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// *Optional*. The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,

    /// *Optional*. True, if the giveaway was completed, but there was no user to win the prize
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_unclaimed: bool,
}

// Divider: all content below this line will be preserved after code regen
