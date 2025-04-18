use serde::{Deserialize, Serialize};

/// This object represents a point on the map.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#location)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// Latitude as defined by the sender
    pub latitude: f64,

    /// Longitude as defined by the sender
    pub longitude: f64,

    /// *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,

    /// *Optional*. Time relative to the message sending date, during which the location can be updated; in seconds. For active live locations only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,

    /// *Optional*. The direction in which user is moving, in degrees; 1-360. For active live locations only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,

    /// *Optional*. The maximum distance for proximity alerts about approaching another chat member, in meters. For sent live locations only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
