use serde::{Deserialize, Serialize};

/// This object describes the rating of a user based on their Telegram Star spendings.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#userrating)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRating {
    /// Current level of the user, indicating their reliability when purchasing digital goods and services. A higher level suggests a more trustworthy customer; a negative level is likely reason for concern.
    pub level: i64,

    /// Numerical value of the user's rating; the higher the rating, the better
    pub rating: i64,

    /// The rating value required to get the current level
    pub current_level_rating: i64,

    /// *Optional*. The rating value required to get to the next level; omitted if the maximum level was reached
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_level_rating: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
