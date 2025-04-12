use serde::Serialize;

use crate::{
    entities::misc::input_file::{GetFiles, InputFile},
    utils::deserialize_utils::is_false,
};

/// Describes a video to post as a story.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputstorycontentvideo)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InputStoryContentVideo {
    /// The video to post as a story. The video must be of the size 720x1280, streamable, encoded with H.265 codec, with key frames added each second in the MPEG4 format, and must not exceed 30 MB. The video can't be reused and can only be uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the video was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub video: InputFile,

    /// *Optional*. Precise duration of the video in seconds; 0-60
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// *Optional*. Timestamp in seconds of the frame that will be used as the static cover for the story. Defaults to 0.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_frame_timestamp: Option<f64>,

    /// *Optional*. Pass *True* if the video has no sound
    #[serde(skip_serializing_if = "is_false")]
    pub is_animation: bool,
}

impl GetFiles for InputStoryContentVideo {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        self.video.form(form).await
    }
}
// Divider: all content below this line will be preserved after code regen
