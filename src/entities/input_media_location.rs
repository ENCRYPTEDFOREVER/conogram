use serde::Serialize;

/// Represents a location to be sent.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputmedialocation)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InputMediaLocation {
    /// Latitude of the location
    pub latitude: f64,

    /// Longitude of the location
    pub longitude: f64,

    /// *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
}

// Divider: all content below this line will be preserved after code regen
