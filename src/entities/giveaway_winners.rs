use serde::{Deserialize, Serialize};

use crate::{
    entities::{chat::Chat, user::User},
    utils::deserialize_utils::is_false,
};

/// This object represents a message about the completion of a giveaway with public winners.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#giveawaywinners)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GiveawayWinners {
    /// The chat that created the giveaway
    pub chat: Box<Chat>,

    /// Identifier of the message with the giveaway in the chat
    pub giveaway_message_id: i64,

    /// Point in time (Unix timestamp) when winners of the giveaway were selected
    pub winners_selection_date: i64,

    /// Total number of winners in the giveaway
    pub winner_count: i64,

    /// List of up to 100 winners of the giveaway
    pub winners: Vec<User>,

    /// *Optional*. The number of other chats the user had to join in order to be eligible for the giveaway
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_chat_count: Option<i64>,

    /// *Optional*. The number of Telegram Stars that were split between giveaway winners; for Telegram Star giveaways only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,

    /// *Optional*. The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i64>,

    /// *Optional*. Number of undistributed prizes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,

    /// *Optional*. *True*, if only users who had joined the chats after the giveaway started were eligible to win
    #[serde(default, skip_serializing_if = "is_false")]
    pub only_new_members: bool,

    /// *Optional*. *True*, if the giveaway was canceled because the payment for it was refunded
    #[serde(default, skip_serializing_if = "is_false")]
    pub was_refunded: bool,

    /// *Optional*. Description of additional giveaway prize
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
