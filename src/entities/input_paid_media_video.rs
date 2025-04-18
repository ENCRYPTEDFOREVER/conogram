use serde::Serialize;

use crate::{
    entities::misc::input_file::{GetFiles, InputFile},
    utils::deserialize_utils::is_false,
};

/// The paid media to send is a video.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputpaidmediavideo)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InputPaidMediaVideo {
    /// File to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub media: InputFile,

    /// *Optional*. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,

    /// *Optional*. Cover for the video in the message. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<InputFile>,

    /// *Optional*. Start timestamp for the video in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,

    /// *Optional*. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// *Optional*. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// *Optional*. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    /// *Optional*. Pass *True* if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "is_false")]
    pub supports_streaming: bool,
}

impl GetFiles for InputPaidMediaVideo {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        let mut form = form;
        form = self.cover.form(form).await?;
        form = self.media.form(form).await?;
        form = self.thumbnail.form(form).await?;
        Ok(form)
    }
}
// Divider: all content below this line will be preserved after code regen
