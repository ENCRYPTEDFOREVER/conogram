use crate::entities::reaction_type::ReactionType;
use serde::{Deserialize, Serialize};

/// Represents a reaction added to a message along with the number of times it was added.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#reactioncount)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReactionCount {
    /// Type of the reaction
    #[serde(rename = "type")]
    pub type_: ReactionType,

    /// Number of times the reaction was added
    pub total_count: i64,
}

// Divider: all content below this line will be preserved after code regen
