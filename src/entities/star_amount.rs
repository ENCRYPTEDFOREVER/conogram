use serde::{Deserialize, Serialize};

/// Describes an amount of Telegram Stars.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#staramount)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarAmount {
    /// Integer amount of Telegram Stars, rounded to 0; can be negative
    pub amount: i64,

    /// *Optional*. The number of 1/1000000000 shares of Telegram Stars; from -999999999 to 999999999; can be negative if and only if *amount* is non-positive
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
