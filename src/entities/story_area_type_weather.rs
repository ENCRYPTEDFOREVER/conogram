use serde::{Deserialize, Serialize};

/// Describes a story area containing weather information. Currently, a story can have up to 3 weather areas.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#storyareatypeweather)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StoryAreaTypeWeather {
    /// Temperature, in degree Celsius
    pub temperature: f64,

    /// Emoji representing the weather
    pub emoji: String,

    /// A color of the area background in the ARGB format
    pub background_color: i64,
}

// Divider: all content below this line will be preserved after code regen
