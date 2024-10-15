use serde::{Deserialize, Serialize};

use crate::entities::background_type::BackgroundType;

/// This object represents a chat background.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatbackground)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatBackground {
    /// Type of the background
    #[serde(rename = "type")]
    pub type_: BackgroundType,
}

// Divider: all content below this line will be preserved after code regen
