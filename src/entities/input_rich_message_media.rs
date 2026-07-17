use serde::Serialize;

use crate::entities::{
    input_media_animation::InputMediaAnimation, input_media_audio::InputMediaAudio,
    input_media_photo::InputMediaPhoto, input_media_video::InputMediaVideo,
    input_media_voice_note::InputMediaVoiceNote,
};

/// Describes a media element embedded in an outgoing rich message.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichmessagemedia)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InputRichMessageMedia {
    /// Unique identifier of the media used in a `tg://photo?id=`, `tg://video?id=`, or `tg://audio?id=` link. 1-64 characters, only `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.
    pub id: String,

    /// The media to be sent. Everything except the media itself and its properties is ignored.
    pub media: Vec<InputRichMedia>,
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::misc::input_file::GetFiles;

/// Describes a media element embedded in an outgoing rich message.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichmessagemedia)
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum InputRichMedia {
    /// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediaanimation)
    #[serde(rename = "animation")]
    Animation(InputMediaAnimation),

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

    /// Represents a voice message file to be sent.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputmediavoicenote)
    #[serde(rename = "voice_note")]
    VoiceNote(InputMediaVoiceNote),
}

impl Default for InputRichMedia {
    fn default() -> Self {
        Self::Animation(InputMediaAnimation::default())
    }
}

impl From<InputMediaAnimation> for InputRichMedia {
    fn from(value: InputMediaAnimation) -> Self {
        Self::Animation(value)
    }
}

impl From<InputMediaAudio> for InputRichMedia {
    fn from(value: InputMediaAudio) -> Self {
        Self::Audio(value)
    }
}

impl From<InputMediaPhoto> for InputRichMedia {
    fn from(value: InputMediaPhoto) -> Self {
        Self::Photo(value)
    }
}

impl From<InputMediaVideo> for InputRichMedia {
    fn from(value: InputMediaVideo) -> Self {
        Self::Video(value)
    }
}

impl From<InputMediaVoiceNote> for InputRichMedia {
    fn from(value: InputMediaVoiceNote) -> Self {
        Self::VoiceNote(value)
    }
}

impl GetFiles for InputRichMedia {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        match self {
            Self::Animation(m) => m.form(form).await,
            Self::Audio(m) => m.form(form).await,
            Self::Photo(m) => m.form(form).await,
            Self::Video(m) => m.form(form).await,
            Self::VoiceNote(m) => m.form(form).await,
        }
    }
}
