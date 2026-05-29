use serde::Serialize;

use crate::entities::misc::input_file::{GetFiles, InputFile};

/// The paid media to send is a live photo.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputpaidmedialivephoto)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InputPaidMediaLivePhoto {
    /// Video of the live photo to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended) or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Sending live photos by a URL is currently unsupported.
    pub media: InputFile,

    /// The static photo to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended) or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Sending live photos by a URL is currently unsupported.
    pub photo: InputFile,
}

impl GetFiles for InputPaidMediaLivePhoto {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        let mut form = form;
        form = self.media.form(form).await?;
        form = self.photo.form(form).await?;
        Ok(form)
    }
}
// Divider: all content below this line will be preserved after code regen
