use serde::{Deserialize, Serialize};

use crate::{entities::chat::Chat, utils::deserialize_utils::is_false};

/// This object represents a message about a scheduled giveaway.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#giveaway)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Giveaway {
    /// The list of chats which the user must join to participate in the giveaway
    pub chats: Vec<Chat>,

    /// Point in time (Unix timestamp) when winners of the giveaway will be selected
    pub winners_selection_date: i64,

    /// The number of users which are supposed to be selected as winners of the giveaway
    pub winner_count: i64,

    /// *Optional*. *True*, if only users who join the chats after the giveaway started should be eligible to win
    #[serde(default, skip_serializing_if = "is_false")]
    pub only_new_members: bool,

    /// *Optional*. *True*, if the list of giveaway winners will be visible to everyone
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_public_winners: bool,

    /// *Optional*. Description of additional giveaway prize
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,

    /// *Optional*. A list of two-letter [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country_codes: Vec<String>,

    /// *Optional*. The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,

    /// *Optional*. The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
