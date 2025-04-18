use serde::{Deserialize, Serialize};

use crate::entities::{
    story_area_type_link::StoryAreaTypeLink, story_area_type_location::StoryAreaTypeLocation,
    story_area_type_suggested_reaction::StoryAreaTypeSuggestedReaction,
    story_area_type_unique_gift::StoryAreaTypeUniqueGift,
    story_area_type_weather::StoryAreaTypeWeather,
};

/// Describes the type of a clickable area on a story. Currently, it can be one of
///
/// * [StoryAreaTypeLocation](https://core.telegram.org/bots/api/#storyareatypelocation)
/// * [StoryAreaTypeSuggestedReaction](https://core.telegram.org/bots/api/#storyareatypesuggestedreaction)
/// * [StoryAreaTypeLink](https://core.telegram.org/bots/api/#storyareatypelink)
/// * [StoryAreaTypeWeather](https://core.telegram.org/bots/api/#storyareatypeweather)
/// * [StoryAreaTypeUniqueGift](https://core.telegram.org/bots/api/#storyareatypeuniquegift)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#storyareatype)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StoryAreaType {
    /// Describes a story area pointing to a location. Currently, a story can have up to 10 location areas.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#storyareatypelocation)
    #[serde(rename = "location")]
    Location(StoryAreaTypeLocation),

    /// Describes a story area pointing to a suggested reaction. Currently, a story can have up to 5 suggested reaction areas.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#storyareatypesuggestedreaction)
    #[serde(rename = "suggested_reaction")]
    SuggestedReaction(StoryAreaTypeSuggestedReaction),

    /// Describes a story area pointing to an HTTP or tg:// link. Currently, a story can have up to 3 link areas.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#storyareatypelink)
    #[serde(rename = "link")]
    Link(StoryAreaTypeLink),

    /// Describes a story area containing weather information. Currently, a story can have up to 3 weather areas.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#storyareatypeweather)
    #[serde(rename = "weather")]
    Weather(StoryAreaTypeWeather),

    /// Describes a story area pointing to a unique gift. Currently, a story can have at most 1 unique gift area.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#storyareatypeuniquegift)
    #[serde(rename = "unique_gift")]
    UniqueGift(StoryAreaTypeUniqueGift),
}

impl Default for StoryAreaType {
    fn default() -> Self {
        Self::Location(StoryAreaTypeLocation::default())
    }
}

impl From<StoryAreaTypeLocation> for StoryAreaType {
    fn from(value: StoryAreaTypeLocation) -> Self {
        Self::Location(value)
    }
}

impl From<StoryAreaTypeSuggestedReaction> for StoryAreaType {
    fn from(value: StoryAreaTypeSuggestedReaction) -> Self {
        Self::SuggestedReaction(value)
    }
}

impl From<StoryAreaTypeLink> for StoryAreaType {
    fn from(value: StoryAreaTypeLink) -> Self {
        Self::Link(value)
    }
}

impl From<StoryAreaTypeWeather> for StoryAreaType {
    fn from(value: StoryAreaTypeWeather) -> Self {
        Self::Weather(value)
    }
}

impl From<StoryAreaTypeUniqueGift> for StoryAreaType {
    fn from(value: StoryAreaTypeUniqueGift) -> Self {
        Self::UniqueGift(value)
    }
}

// Divider: all content below this line will be preserved after code regen
