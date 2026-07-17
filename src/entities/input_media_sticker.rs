use serde::Serialize;

use crate::entities::misc::input_file::{GetFiles, InputFile};

/// Represents a sticker file to be sent.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputmediasticker)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename = "sticker", tag = "type")]
pub struct InputMediaSticker {
    /// File to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a .WEBP sticker from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub media: InputFile,

    /// *Optional*. Emoji associated with the sticker; only for just uploaded stickers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
}

impl GetFiles for InputMediaSticker {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        self.media.form(form).await
    }
}
// Divider: all content below this line will be preserved after code regen
