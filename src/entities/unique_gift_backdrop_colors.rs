use serde::{Deserialize, Serialize};

/// This object describes the colors of the backdrop of a unique gift.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#uniquegiftbackdropcolors)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniqueGiftBackdropColors {
    /// The color in the center of the backdrop in RGB format
    pub center_color: i64,

    /// The color on the edges of the backdrop in RGB format
    pub edge_color: i64,

    /// The color to be applied to the symbol in RGB format
    pub symbol_color: i64,

    /// The color for the text on the backdrop in RGB format
    pub text_color: i64,
}

// Divider: all content below this line will be preserved after code regen
