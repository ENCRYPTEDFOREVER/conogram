use serde::Serialize;

/// Represents a venue to be sent.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputmediavenue)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InputMediaVenue {
    /// Latitude of the location
    pub latitude: f64,

    /// Longitude of the location
    pub longitude: f64,

    /// Name of the venue
    pub title: String,

    /// Address of the venue
    pub address: String,

    /// *Optional*. Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,

    /// *Optional*. Foursquare type of the venue, if known. (For example, “arts\_entertainment/default”, “arts\_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,

    /// *Optional*. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,

    /// *Optional*. Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
