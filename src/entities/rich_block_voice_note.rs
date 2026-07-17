use serde::{Deserialize, Serialize};

use crate::entities::{rich_block_caption::RichBlockCaption, voice::Voice};

/// A block with a voice note, corresponding to the HTML tag `<audio>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockvoicenote)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "voice_note", tag = "type")]
pub struct RichBlockVoiceNote {
    /// The voice note
    pub voice_note: Voice,

    /// *Optional*. Caption of the block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen
