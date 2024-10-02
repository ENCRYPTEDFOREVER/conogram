use serde::{Deserialize, Serialize};

/// This object represents a service message about a user boosting a chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatboostadded)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatBoostAdded {
    /// Number of boosts added by the user
    pub boost_count: i64,
}

// Divider: all content below this line will be preserved after code regen
