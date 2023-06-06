use crate::entities::location::Location;
use serde::{Deserialize, Serialize};

///This object represents a venue.
///API Reference: [link](https://core.telegram.org/bots/api/#venue)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Venue {
    ///Venue location. Can't be a live location
    pub location: Location,

    ///Name of the venue
    pub title: String,

    ///Address of the venue
    pub address: String,

    ///*Optional*. Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,

    ///*Optional*. Foursquare type of the venue. (For example, “arts\_entertainment/default”, “arts\_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,

    ///*Optional*. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,

    ///*Optional*. Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}
// Divider: all content below this line will be preserved after code regen
