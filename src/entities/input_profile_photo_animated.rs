use serde::Serialize;

use crate::entities::misc::input_file::{GetFiles, InputFile};

/// An animated profile photo in the MPEG4 format.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputprofilephotoanimated)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InputProfilePhotoAnimated {
    /// The animated profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the photo was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub animation: InputFile,

    /// *Optional*. Timestamp in seconds of the frame that will be used as the static profile photo. Defaults to 0.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_frame_timestamp: Option<f64>,
}

impl GetFiles for InputProfilePhotoAnimated {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        self.animation.form(form).await
    }
}
// Divider: all content below this line will be preserved after code regen
