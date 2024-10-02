use crate::entities::misc::input_file::GetFiles;
use crate::entities::misc::input_file::InputFile;
use crate::entities::misc::input_file::Moose;
use serde::Serialize;
use std::collections::HashMap;

/// The paid media to send is a photo.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputpaidmediaphoto)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InputPaidMediaPhoto {
    /// File to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub media: InputFile,
}

impl GetFiles for InputPaidMediaPhoto {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        map.insert(
            Moose::Owned(self.media.get_uuid().unwrap_or_else(|| "media".into())),
            &self.media,
        );
        map
    }
}
// Divider: all content below this line will be preserved after code regen
