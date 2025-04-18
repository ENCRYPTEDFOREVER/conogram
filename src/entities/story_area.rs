use serde::{Deserialize, Serialize};

use crate::entities::{story_area_position::StoryAreaPosition, story_area_type::StoryAreaType};

/// Describes a clickable area on a story media.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#storyarea)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StoryArea {
    /// Position of the area
    pub position: StoryAreaPosition,

    /// Type of the area
    #[serde(rename = "type")]
    pub type_: StoryAreaType,
}

// Divider: all content below this line will be preserved after code regen
