use serde::Serialize;

use crate::entities::{
    input_paid_media_photo::InputPaidMediaPhoto,
    input_paid_media_video::InputPaidMediaVideo,
    misc::input_file::{GetFiles, InputFile},
};

/// This object describes the paid media to be sent. Currently, it can be one of
///
/// * [InputPaidMediaPhoto](https://core.telegram.org/bots/api/#inputpaidmediaphoto)
/// * [InputPaidMediaVideo](https://core.telegram.org/bots/api/#inputpaidmediavideo)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputpaidmedia)
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum InputPaidMedia {
    /// The paid media to send is a photo.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputpaidmediaphoto)
    #[serde(rename = "photo")]
    Photo(InputPaidMediaPhoto),

    /// The paid media to send is a video.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputpaidmediavideo)
    #[serde(rename = "video")]
    Video(InputPaidMediaVideo),
}

impl Default for InputPaidMedia {
    fn default() -> Self {
        Self::Photo(InputPaidMediaPhoto::default())
    }
}

impl From<InputPaidMediaPhoto> for InputPaidMedia {
    fn from(value: InputPaidMediaPhoto) -> Self {
        Self::Photo(value)
    }
}

impl From<InputPaidMediaVideo> for InputPaidMedia {
    fn from(value: InputPaidMediaVideo) -> Self {
        Self::Video(value)
    }
}

impl GetFiles for InputPaidMedia {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        match self {
            Self::Photo(m) => m.form(form).await,
            Self::Video(m) => m.form(form).await,
        }
    }
}
// Divider: all content below this line will be preserved after code regen
impl<T: Into<InputFile>> From<T> for InputPaidMediaPhoto {
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
        }
    }
}

impl<T: Into<InputFile>> From<T> for InputPaidMediaVideo {
    fn from(value: T) -> Self {
        Self {
            media: value.into(),
            ..Default::default()
        }
    }
}
