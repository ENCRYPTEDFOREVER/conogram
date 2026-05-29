use serde::{Deserialize, Serialize};

/// This object describes the background of a gift.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#giftbackground)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GiftBackground {
    /// Center color of the background in RGB format
    pub center_color: i64,

    /// Edge color of the background in RGB format
    pub edge_color: i64,

    /// Text color of the background in RGB format
    pub text_color: i64,
}

// Divider: all content below this line will be preserved after code regen
