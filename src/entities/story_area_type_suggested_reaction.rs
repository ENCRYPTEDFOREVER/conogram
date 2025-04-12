use serde::{Deserialize, Serialize};

use crate::{entities::reaction_type::ReactionType, utils::deserialize_utils::is_false};

/// Describes a story area pointing to a suggested reaction. Currently, a story can have up to 5 suggested reaction areas.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#storyareatypesuggestedreaction)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryAreaTypeSuggestedReaction {
    /// Type of the reaction
    pub reaction_type: ReactionType,

    /// *Optional*. Pass *True* if the reaction area has a dark background
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_dark: bool,

    /// *Optional*. Pass *True* if reaction area corner is flipped
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_flipped: bool,
}

// Divider: all content below this line will be preserved after code regen
