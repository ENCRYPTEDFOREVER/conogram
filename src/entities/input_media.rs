use crate::entities::input_media_animation::InputMediaAnimation;
use crate::entities::input_media_audio::InputMediaAudio;
use crate::entities::input_media_document::InputMediaDocument;
use crate::entities::input_media_photo::InputMediaPhoto;
use crate::entities::input_media_video::InputMediaVideo;
use crate::entities::misc::input_file::GetFiles;
use crate::entities::misc::input_file::InputFile;
use crate::entities::misc::input_file::Moose;
use serde::Serialize;
use std::collections::HashMap;

/// This object represents the content of a media message to be sent. It should be one of
///
/// * [InputMediaAnimation](https://core.telegram.org/bots/api/#inputmediaanimation)
/// * [InputMediaDocument](https://core.telegram.org/bots/api/#inputmediadocument)
/// * [InputMediaAudio](https://core.telegram.org/bots/api/#inputmediaaudio)
/// * [InputMediaPhoto](https://core.telegram.org/bots/api/#inputmediaphoto)
/// * [InputMediaVideo](https://core.telegram.org/bots/api/#inputmediavideo)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputmedia)
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum InputMedia {
    #[serde(rename = "animation")]
    Animation(InputMediaAnimation),

    #[serde(rename = "document")]
    Document(InputMediaDocument),

    #[serde(rename = "audio")]
    Audio(InputMediaAudio),

    #[serde(rename = "photo")]
    Photo(InputMediaPhoto),

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
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
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
