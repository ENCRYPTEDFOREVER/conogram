use serde::Serialize;

use crate::entities::{
    input_media_animation::InputMediaAnimation, input_media_audio::InputMediaAudio,
    input_media_document::InputMediaDocument, input_media_photo::InputMediaPhoto,
    input_media_video::InputMediaVideo, misc::input_file::GetFiles,
};

/// This object represents the content of a media message to be sent. It should be one of
///
/// * [InputMediaAnimation](https://core.telegram.org/bots/api/#inputmediaanimation)
/// * [InputMediaDocument](https://core.telegram.org/bots/api/#inputmediadocument)
/// * [InputMediaAudio](https://core.telegram.org/bots/api/#inputmediaaudio)
/// * [InputMediaPhoto](https://core.telegram.org/bots/api/#inputmediaphoto)
/// * [InputMediaVideo](https://core.telegram.org/bots/api/#inputmediavideo)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputmedia)
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum InputMedia {
    /// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediaanimation)
    #[serde(rename = "animation")]
    Animation(InputMediaAnimation),

    /// Represents a general file to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediadocument)
    #[serde(rename = "document")]
    Document(InputMediaDocument),

    /// Represents an audio file to be treated as music to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediaaudio)
    #[serde(rename = "audio")]
    Audio(InputMediaAudio),

    /// Represents a photo to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediaphoto)
    #[serde(rename = "photo")]
    Photo(InputMediaPhoto),

    /// Represents a video to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediavideo)
    #[serde(rename = "video")]
    Video(InputMediaVideo),
}

impl Default for InputMedia {
    fn default() -> Self {
        Self::Animation(InputMediaAnimation::default())
    }
}

impl From<InputMediaAnimation> for InputMedia {
    fn from(value: InputMediaAnimation) -> Self {
        Self::Animation(value)
    }
}

impl From<InputMediaDocument> for InputMedia {
    fn from(value: InputMediaDocument) -> Self {
        Self::Document(value)
    }
}

impl From<InputMediaAudio> for InputMedia {
    fn from(value: InputMediaAudio) -> Self {
        Self::Audio(value)
    }
}

impl From<InputMediaPhoto> for InputMedia {
    fn from(value: InputMediaPhoto) -> Self {
        Self::Photo(value)
    }
}

impl From<InputMediaVideo> for InputMedia {
    fn from(value: InputMediaVideo) -> Self {
        Self::Video(value)
    }
}

impl GetFiles for InputMedia {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        match self {
            Self::Animation(m) => m.form(form).await,
            Self::Audio(m) => m.form(form).await,
            Self::Document(m) => m.form(form).await,
            Self::Photo(m) => m.form(form).await,
            Self::Video(m) => m.form(form).await,
        }
    }
}
// Divider: all content below this line will be preserved after code regen
use crate::entities::misc::input_file::InputFile;

impl<T: Into<InputFile>> From<T> for InputMediaAnimation {
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            ..Default::default()
        }
    }
}

impl<T: Into<InputFile>> From<T> for InputMediaDocument {
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            ..Default::default()
        }
    }
}
impl<T: Into<InputFile>> From<T> for InputMediaAudio {
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            ..Default::default()
        }
    }
}
impl<T: Into<InputFile>> From<T> for InputMediaPhoto {
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            ..Default::default()
        }
    }
}

impl<T: Into<InputFile>> From<T> for InputMediaVideo {
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            ..Default::default()
        }
    }
}
