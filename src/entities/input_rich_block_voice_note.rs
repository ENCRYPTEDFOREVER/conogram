use serde::Serialize;

use crate::entities::{
    input_media_voice_note::InputMediaVoiceNote, rich_block_caption::RichBlockCaption,
};

/// A block with a voice note, corresponding to the HTML tag `<audio>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockvoicenote)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "voice_note", tag = "type")]
pub struct InputRichBlockVoiceNote {
    /// The voice note. Caption is ignored.
    pub voice_note: InputMediaVoiceNote,

    /// *Optional*. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::misc::input_file::GetFiles;

impl GetFiles for InputRichBlockVoiceNote {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        self.voice_note.form(form).await
    }
}
