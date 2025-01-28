use serde::Serialize;

use crate::entities::{
    input_media_animation::InputMediaAnimation,
    input_media_audio::InputMediaAudio,
    input_media_document::InputMediaDocument,
    input_media_photo::InputMediaPhoto,
    input_media_video::InputMediaVideo,
    misc::input_file::{GetFiles, InputFile},
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
    fn get_files(&self) -> Vec<&InputFile> {
        match self {
            Self::Animation(m) => m.get_files(),
            Self::Audio(m) => m.get_files(),
            Self::Document(m) => m.get_files(),
            Self::Photo(m) => m.get_files(),
            Self::Video(m) => m.get_files(),
        }
    }
}
// Divider: all content below this line will be preserved after code regen
