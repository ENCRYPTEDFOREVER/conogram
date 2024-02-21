use crate::entities::mask_position::MaskPosition;
use crate::entities::misc::input_file::InputFile;
use serde::Serialize;

///This object describes a sticker to be added to a sticker set.
///API Reference: [link](https://core.telegram.org/bots/api/#inputsticker)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InputSticker {
    ///The added sticker. Pass a *file\_id* as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, upload a new one using multipart/form-data, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. Animated and video stickers can't be uploaded via HTTP URL. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub sticker: InputFile,

    ///List of 1-20 emoji associated with the sticker
    pub emoji_list: Vec<String>,

    ///*Optional*. Position where the mask should be placed on faces. For “mask” stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,

    ///*Optional*. List of 0-20 search keywords for the sticker with total length of up to 64 characters. For “regular” and “custom\_emoji” stickers only.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
}
// Divider: all content below this line will be preserved after code regen
use super::misc::input_file::{GetFiles, Moose};
use std::collections::HashMap;

impl GetFiles for InputSticker {
    fn get_files(&self) -> std::collections::HashMap<super::misc::input_file::Moose, &InputFile> {
        let mut map = HashMap::new();
        map.insert(
            Moose::Owned(self.sticker.get_uuid().unwrap_or_else(|| "sticker".into())),
            &self.sticker,
        );
        map
    }
}
