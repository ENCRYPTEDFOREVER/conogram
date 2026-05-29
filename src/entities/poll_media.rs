use serde::{Deserialize, Serialize};

use crate::entities::{
    animation::Animation, audio::Audio, document::Document, live_photo::LivePhoto,
    location::Location, photo_size::PhotoSize, sticker::Sticker, venue::Venue, video::Video,
};

/// At most **one** of the optional fields can be present in any given object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#pollmedia)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PollMedia {
    /// *Optional*. Media is an animation, information about the animation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,

    /// *Optional*. Media is an audio file, information about the file; currently, can't be received in a poll option
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    /// *Optional*. Media is a general file, information about the file; currently, can't be received in a poll option
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,

    /// *Optional*. Media is a live photo, information about the live photo
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_photo: Option<LivePhoto>,

    /// *Optional*. Media is a shared location, information about the location
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    /// *Optional*. Media is a photo, available sizes of the photo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub photo: Vec<PhotoSize>,

    /// *Optional*. Media is a sticker, information about the sticker; currently, for poll options only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,

    /// *Optional*. Media is a venue, information about the venue
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,

    /// *Optional*. Media is a video, information about the video
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
}

// Divider: all content below this line will be preserved after code regen
