use serde::Serialize;

use crate::entities::{
    input_media_animation::InputMediaAnimation, input_media_audio::InputMediaAudio,
    input_media_document::InputMediaDocument, input_media_live_photo::InputMediaLivePhoto,
    input_media_location::InputMediaLocation, input_media_photo::InputMediaPhoto,
    input_media_venue::InputMediaVenue, input_media_video::InputMediaVideo,
    misc::input_file::GetFiles,
};

/// This object represents the content of a poll description or a quiz explanation to be sent. It should be one of
///
/// * [InputMediaAnimation](https://core.telegram.org/bots/api/#inputmediaanimation)
/// * [InputMediaAudio](https://core.telegram.org/bots/api/#inputmediaaudio)
/// * [InputMediaDocument](https://core.telegram.org/bots/api/#inputmediadocument)
/// * [InputMediaLivePhoto](https://core.telegram.org/bots/api/#inputmedialivephoto)
/// * [InputMediaLocation](https://core.telegram.org/bots/api/#inputmedialocation)
/// * [InputMediaPhoto](https://core.telegram.org/bots/api/#inputmediaphoto)
/// * [InputMediaVenue](https://core.telegram.org/bots/api/#inputmediavenue)
/// * [InputMediaVideo](https://core.telegram.org/bots/api/#inputmediavideo)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputpollmedia)
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum InputPollMedia {
    /// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediaanimation)
    #[serde(rename = "animation")]
    MediaAnimation(InputMediaAnimation),

    /// Represents an audio file to be treated as music to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediaaudio)
    #[serde(rename = "audio")]
    MediaAudio(InputMediaAudio),

    /// Represents a general file to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediadocument)
    #[serde(rename = "document")]
    MediaDocument(InputMediaDocument),

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

impl Default for InputPollMedia {
    fn default() -> Self {
        Self::MediaAnimation(InputMediaAnimation::default())
    }
}

impl From<InputMediaAnimation> for InputPollMedia {
    fn from(value: InputMediaAnimation) -> Self {
        Self::MediaAnimation(value)
    }
}

impl From<InputMediaAudio> for InputPollMedia {
    fn from(value: InputMediaAudio) -> Self {
        Self::MediaAudio(value)
    }
}

impl From<InputMediaDocument> for InputPollMedia {
    fn from(value: InputMediaDocument) -> Self {
        Self::MediaDocument(value)
    }
}

impl From<InputMediaLivePhoto> for InputPollMedia {
    fn from(value: InputMediaLivePhoto) -> Self {
        Self::MediaLivePhoto(value)
    }
}

impl From<InputMediaLocation> for InputPollMedia {
    fn from(value: InputMediaLocation) -> Self {
        Self::MediaLocation(value)
    }
}

impl From<InputMediaPhoto> for InputPollMedia {
    fn from(value: InputMediaPhoto) -> Self {
        Self::MediaPhoto(value)
    }
}

impl From<InputMediaVenue> for InputPollMedia {
    fn from(value: InputMediaVenue) -> Self {
        Self::MediaVenue(value)
    }
}

impl From<InputMediaVideo> for InputPollMedia {
    fn from(value: InputMediaVideo) -> Self {
        Self::MediaVideo(value)
    }
}

impl GetFiles for InputPollMedia {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        match self {
            Self::MediaAnimation(m) => m.form(form).await,
            Self::MediaAudio(m) => m.form(form).await,
            Self::MediaDocument(m) => m.form(form).await,
            Self::MediaLivePhoto(m) => m.form(form).await,
            Self::MediaPhoto(m) => m.form(form).await,
            Self::MediaVideo(m) => m.form(form).await,
            Self::MediaLocation(_) | Self::MediaVenue(_) => Ok(form),
        }
    }
}
// Divider: all content below this line will be preserved after code regen
