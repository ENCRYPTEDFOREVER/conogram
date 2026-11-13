use serde::Serialize;

use crate::entities::{
    input_media_animation::InputMediaAnimation, input_media_link::InputMediaLink,
    input_media_live_photo::InputMediaLivePhoto, input_media_location::InputMediaLocation,
    input_media_photo::InputMediaPhoto, input_media_sticker::InputMediaSticker,
    input_media_venue::InputMediaVenue, input_media_video::InputMediaVideo,
    misc::input_file::GetFiles,
};

/// This object represents the content of a poll option to be sent. It should be one of
///
/// * [InputMediaAnimation](https://core.telegram.org/bots/api/#inputmediaanimation)
/// * [InputMediaLink](https://core.telegram.org/bots/api/#inputmedialink)
/// * [InputMediaLivePhoto](https://core.telegram.org/bots/api/#inputmedialivephoto)
/// * [InputMediaLocation](https://core.telegram.org/bots/api/#inputmedialocation)
/// * [InputMediaPhoto](https://core.telegram.org/bots/api/#inputmediaphoto)
/// * [InputMediaSticker](https://core.telegram.org/bots/api/#inputmediasticker)
/// * [InputMediaVenue](https://core.telegram.org/bots/api/#inputmediavenue)
/// * [InputMediaVideo](https://core.telegram.org/bots/api/#inputmediavideo)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputpolloptionmedia)
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum InputPollOptionMedia {
    /// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediaanimation)
    #[serde(rename = "animation")]
    MediaAnimation(InputMediaAnimation),

    /// Represents an HTTP link to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmedialink)
    #[serde(rename = "link")]
    MediaLink(InputMediaLink),

    /// Represents a live photo to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmedialivephoto)
    #[serde(rename = "live_photo")]
    MediaLivePhoto(InputMediaLivePhoto),

    /// Represents a location to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmedialocation)
    #[serde(rename = "location")]
    MediaLocation(InputMediaLocation),

    /// Represents a photo to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediaphoto)
    #[serde(rename = "photo")]
    MediaPhoto(InputMediaPhoto),

    /// Represents a sticker file to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediasticker)
    #[serde(rename = "sticker")]
    MediaSticker(InputMediaSticker),

    /// Represents a venue to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediavenue)
    #[serde(rename = "venue")]
    MediaVenue(InputMediaVenue),

    /// Represents a video to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediavideo)
    #[serde(rename = "video")]
    MediaVideo(InputMediaVideo),
}

impl Default for InputPollOptionMedia {
    fn default() -> Self {
        Self::MediaAnimation(InputMediaAnimation::default())
    }
}

impl From<InputMediaAnimation> for InputPollOptionMedia {
    fn from(value: InputMediaAnimation) -> Self {
        Self::MediaAnimation(value)
    }
}

impl From<InputMediaLink> for InputPollOptionMedia {
    fn from(value: InputMediaLink) -> Self {
        Self::MediaLink(value)
    }
}

impl From<InputMediaLivePhoto> for InputPollOptionMedia {
    fn from(value: InputMediaLivePhoto) -> Self {
        Self::MediaLivePhoto(value)
    }
}

impl From<InputMediaLocation> for InputPollOptionMedia {
    fn from(value: InputMediaLocation) -> Self {
        Self::MediaLocation(value)
    }
}

impl From<InputMediaPhoto> for InputPollOptionMedia {
    fn from(value: InputMediaPhoto) -> Self {
        Self::MediaPhoto(value)
    }
}

impl From<InputMediaSticker> for InputPollOptionMedia {
    fn from(value: InputMediaSticker) -> Self {
        Self::MediaSticker(value)
    }
}

impl From<InputMediaVenue> for InputPollOptionMedia {
    fn from(value: InputMediaVenue) -> Self {
        Self::MediaVenue(value)
    }
}

impl From<InputMediaVideo> for InputPollOptionMedia {
    fn from(value: InputMediaVideo) -> Self {
        Self::MediaVideo(value)
    }
}

impl GetFiles for InputPollOptionMedia {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        match self {
            Self::MediaAnimation(m) => m.form(form).await,
            Self::MediaLivePhoto(m) => m.form(form).await,
            Self::MediaPhoto(m) => m.form(form).await,
            Self::MediaSticker(m) => m.form(form).await,
            Self::MediaVideo(m) => m.form(form).await,
            Self::MediaLink(_) | Self::MediaLocation(_) | Self::MediaVenue(_) => Ok(form),
        }
    }
}
// Divider: all content below this line will be preserved after code regen
