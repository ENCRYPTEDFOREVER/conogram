use crate::entities::input_paid_media_photo::InputPaidMediaPhoto;
use crate::entities::input_paid_media_video::InputPaidMediaVideo;
use crate::entities::misc::input_file::GetFiles;
use crate::entities::misc::input_file::InputFile;
use crate::entities::misc::input_file::Moose;
use serde::Serialize;
use std::collections::HashMap;

/// This object describes the paid media to be sent. Currently, it can be one of
///
/// * [InputPaidMediaPhoto](https://core.telegram.org/bots/api/#inputpaidmediaphoto)
/// * [InputPaidMediaVideo](https://core.telegram.org/bots/api/#inputpaidmediavideo)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputpaidmedia)
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum InputPaidMedia {
    #[serde(rename = "photo")]
    Photo(InputPaidMediaPhoto),

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
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        match self {
            Self::Photo(m) => m.get_files(),
            Self::Video(m) => m.get_files(),
        }
    }
}
// Divider: all content below this line will be preserved after code regen
