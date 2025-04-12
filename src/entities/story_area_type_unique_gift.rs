use serde::{Deserialize, Serialize};

/// Describes a story area pointing to a unique gift. Currently, a story can have at most 1 unique gift area.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#storyareatypeuniquegift)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryAreaTypeUniqueGift {
    /// Unique name of the gift
    pub name: String,
}

// Divider: all content below this line will be preserved after code regen
