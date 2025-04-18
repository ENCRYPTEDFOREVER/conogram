use serde::Serialize;

use crate::entities::misc::input_file::{GetFiles, InputFile};

/// A static profile photo in the .JPG format.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputprofilephotostatic)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InputProfilePhotoStatic {
    /// The static profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the photo was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub photo: InputFile,
}

impl GetFiles for InputProfilePhotoStatic {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        self.photo.form(form).await
    }
}
// Divider: all content below this line will be preserved after code regen
