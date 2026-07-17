use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A block with a “Thinking…” placeholder, corresponding to the custom HTML tag `<tg-thinking>`. The block may be used only in [sendRichMessageDraft](https://core.telegram.org/bots/api/#sendrichmessagedraft), therefore it can't be received in messages. See [https://t.me/addemoji/AIActions](https://t.me/addemoji/AIActions) for examples of custom emoji that are recommended for usage in the block.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockthinking)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "thinking", tag = "type")]
pub struct RichBlockThinking {
    /// Text of the block. See [https://t.me/addemoji/AIActions](https://t.me/addemoji/AIActions) for examples of custom emoji that are recommended for usage in the block.
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
