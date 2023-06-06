use crate::entities::input_media_animation::InputMediaAnimation;
use crate::entities::input_media_audio::InputMediaAudio;
use crate::entities::input_media_document::InputMediaDocument;
use crate::entities::input_media_photo::InputMediaPhoto;
use crate::entities::input_media_video::InputMediaVideo;
use serde::Serialize;

///This object represents the content of a media message to be sent. It should be one of
///
///* [InputMediaAnimation](https://core.telegram.org/bots/api/#inputmediaanimation)
///* [InputMediaDocument](https://core.telegram.org/bots/api/#inputmediadocument)
///* [InputMediaAudio](https://core.telegram.org/bots/api/#inputmediaaudio)
///* [InputMediaPhoto](https://core.telegram.org/bots/api/#inputmediaphoto)
///* [InputMediaVideo](https://core.telegram.org/bots/api/#inputmediavideo)
///API Reference: [link](https://core.telegram.org/bots/api/#inputmedia)
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
// Divider: all content below this line will be preserved after code regen

use super::misc::input_file::GetFiles;

impl GetFiles for InputMedia {
    fn get_files(
        &self,
    ) -> std::collections::HashMap<
        super::misc::input_file::Moose,
        &super::misc::input_file::InputFile,
    > {
        match self {
            InputMedia::Animation(m) => m.get_files(),
            InputMedia::Document(m) => m.get_files(),
            InputMedia::Audio(m) => m.get_files(),
            InputMedia::Photo(m) => m.get_files(),
            InputMedia::Video(m) => m.get_files(),
        }
    }
}
