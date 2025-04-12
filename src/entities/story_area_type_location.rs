use serde::{Deserialize, Serialize};

use crate::entities::location_address::LocationAddress;

/// Describes a story area pointing to a location. Currently, a story can have up to 10 location areas.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#storyareatypelocation)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StoryAreaTypeLocation {
    /// Location latitude in degrees
    pub latitude: f64,

    /// Location longitude in degrees
    pub longitude: f64,

    /// *Optional*. Address of the location
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<LocationAddress>,
}

// Divider: all content below this line will be preserved after code regen
