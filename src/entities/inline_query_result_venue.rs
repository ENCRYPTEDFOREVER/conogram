use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::entities::input_message_content::InputMessageContent;
use serde::Serialize;

///Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the venue.
///API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultvenue)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InlineQueryResultVenue {
    ///Unique identifier for this result, 1-64 Bytes
    pub id: String,

    ///Latitude of the venue location in degrees
    pub latitude: f64,

    ///Longitude of the venue location in degrees
    pub longitude: f64,

    ///Title of the venue
    pub title: String,

    ///Address of the venue
    pub address: String,

    ///*Optional*. Foursquare identifier of the venue if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,

    ///*Optional*. Foursquare type of the venue, if known. (For example, “arts\_entertainment/default”, “arts\_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,

    ///*Optional*. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,

    ///*Optional*. Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,

    ///*Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    ///*Optional*. Content of the message to be sent instead of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    ///*Optional*. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,

    ///*Optional*. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,

    ///*Optional*. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i64>,
}
// Divider: all content below this line will be preserved after code regen
