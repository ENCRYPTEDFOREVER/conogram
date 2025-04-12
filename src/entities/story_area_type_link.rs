use serde::{Deserialize, Serialize};

/// Describes a story area pointing to an HTTP or tg:// link. Currently, a story can have up to 3 link areas.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#storyareatypelink)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryAreaTypeLink {
    /// HTTP or tg:// URL to be opened when the area is clicked
    pub url: String,
}

// Divider: all content below this line will be preserved after code regen
