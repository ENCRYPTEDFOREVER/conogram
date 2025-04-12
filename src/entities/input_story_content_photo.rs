use serde::Serialize;

use crate::entities::misc::input_file::{GetFiles, InputFile};

/// Describes a photo to post as a story.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputstorycontentphoto)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InputStoryContentPhoto {
    /// The photo to post as a story. The photo must be of the size 1080x1920 and must not exceed 10 MB. The photo can't be reused and can only be uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the photo was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub photo: InputFile,
}

impl GetFiles for InputStoryContentPhoto {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        self.photo.form(form).await
    }
}
// Divider: all content below this line will be preserved after code regen
