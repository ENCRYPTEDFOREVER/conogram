use serde::{Deserialize, Serialize};

use crate::{entities::rich_block::RichBlock, utils::deserialize_utils::is_false};

/// Rich formatted message.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richmessage)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichMessage {
    /// Content of the message
    pub blocks: Vec<RichBlock>,

    /// *Optional*. *True*, if the rich message must be shown right-to-left
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_rtl: bool,
}

// Divider: all content below this line will be preserved after code regen
