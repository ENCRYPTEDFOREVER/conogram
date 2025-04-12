use serde::Serialize;

use crate::entities::{
    input_story_content_photo::InputStoryContentPhoto,
    input_story_content_video::InputStoryContentVideo, misc::input_file::GetFiles,
};

/// This object describes the content of a story to post. Currently, it can be one of
///
/// * [InputStoryContentPhoto](https://core.telegram.org/bots/api/#inputstorycontentphoto)
/// * [InputStoryContentVideo](https://core.telegram.org/bots/api/#inputstorycontentvideo)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputstorycontent)
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum InputStoryContent {
    /// Describes a photo to post as a story.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputstorycontentphoto)
    #[serde(rename = "photo")]
    Photo(InputStoryContentPhoto),

    /// Describes a video to post as a story.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputstorycontentvideo)
    #[serde(rename = "video")]
    Video(InputStoryContentVideo),
}

impl Default for InputStoryContent {
    fn default() -> Self {
        Self::Photo(InputStoryContentPhoto::default())
    }
}

impl From<InputStoryContentPhoto> for InputStoryContent {
    fn from(value: InputStoryContentPhoto) -> Self {
        Self::Photo(value)
    }
}

impl From<InputStoryContentVideo> for InputStoryContent {
    fn from(value: InputStoryContentVideo) -> Self {
        Self::Video(value)
    }
}

impl GetFiles for InputStoryContent {
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
