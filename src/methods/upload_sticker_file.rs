use serde::Serialize;

use crate::entities::misc::input_file::InputFile;

#[derive(Debug, Clone, Serialize)]
///Use this method to upload a file with a sticker for later use in the [createNewStickerSet](https://core.telegram.org/bots/api/#createnewstickerset), [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), or [replaceStickerInSet](https://core.telegram.org/bots/api/#replacestickerinset) methods (the file can be used multiple times). Returns the uploaded [File](https://core.telegram.org/bots/api/#file) on success.
pub struct UploadStickerFileParams {
    ///User identifier of sticker file owner
    pub user_id: i64,

    ///A file with the sticker in .WEBP, .PNG, .TGS, or .WEBM format. See [https://core.telegram.org/stickers](https://core.telegram.org/stickers) for technical requirements. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub sticker: InputFile,

    ///Format of the sticker, must be one of “static”, “animated”, “video”
    pub sticker_format: UploadStickerFileStickerFormat,
}

///Format of the sticker, must be one of “static”, “animated”, “video”
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename = "sticker_format")]
pub enum UploadStickerFileStickerFormat {
    #[default]
    /// "static"
    #[serde(rename = "static")]
    Static,

    /// "animated"
    #[serde(rename = "animated")]
    Animated,

    /// "video"
    #[serde(rename = "video")]
    Video,
}

// Divider: all content below this line will be preserved after code regen
