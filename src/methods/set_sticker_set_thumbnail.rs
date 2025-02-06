use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{input_sticker::InputStickerFormat, misc::input_file::InputFile};

/// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setstickersetthumbnail)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetStickerSetThumbnailParams {
    /// Sticker set name
    pub name: String,

    /// User identifier of the sticker set owner
    pub user_id: i64,

    /// A **.WEBP** or **.PNG** image with the thumbnail, must be up to 128 kilobytes in size and have a width and height of exactly 100px, or a **.TGS** animation with a thumbnail up to 32 kilobytes in size (see [https://core.telegram.org/stickers#animation-requirements](https://core.telegram.org/stickers#animation-requirements) for animated sticker technical requirements), or a **.WEBM** video with the thumbnail up to 32 kilobytes in size; see [https://core.telegram.org/stickers#video-requirements](https://core.telegram.org/stickers#video-requirements) for video sticker technical requirements. Pass a *file\_id* as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Animated and video sticker set thumbnails can't be uploaded via HTTP URL. If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,

    /// Format of the thumbnail, must be one of “static” for a **.WEBP** or **.PNG** image, “animated” for a **.TGS** animation, or “video” for a **.WEBM** video
    pub format: InputStickerFormat,
}

// Divider: all content below this line will be preserved after code regen
