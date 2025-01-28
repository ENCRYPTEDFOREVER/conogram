use serde::Serialize;

use crate::entities::{
    mask_position::MaskPosition,
    misc::input_file::{GetFiles, InputFile},
};

/// This object describes a sticker to be added to a sticker set.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputsticker)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InputSticker {
    /// The added sticker. Pass a *file\_id* as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, upload a new one using multipart/form-data, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. Animated and video stickers can't be uploaded via HTTP URL. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub sticker: InputFile,

    /// Format of the added sticker, must be one of “static” for a **.WEBP** or **.PNG** image, “animated” for a **.TGS** animation, “video” for a **WEBM** video
    pub format: InputStickerFormat,

    /// List of 1-20 emoji associated with the sticker
    pub emoji_list: Vec<String>,

    /// *Optional*. Position where the mask should be placed on faces. For “mask” stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,

    /// *Optional*. List of 0-20 search keywords for the sticker with total length of up to 64 characters. For “regular” and “custom\_emoji” stickers only.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
}

/// Format of the added sticker, must be one of “static” for a **.WEBP** or **.PNG** image, “animated” for a **.TGS** animation, “video” for a **WEBM** video
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub enum InputStickerFormat {
    /// `static`
    #[default]
    #[serde(rename = "static")]
    Static,

    /// `animated`
    #[serde(rename = "animated")]
    Animated,

    /// `video`
    #[serde(rename = "video")]
    Video,
}

impl GetFiles for InputSticker {
    fn get_files(&self) -> Vec<&InputFile> {
        vec![&self.sticker]
    }
} // Divider: all content below this line will be preserved after code regen
