use serde::{Deserialize, Serialize};

/// This object contains information about the color scheme for a user's name, message replies and link previews based on a unique gift.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#uniquegiftcolors)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniqueGiftColors {
    /// Custom emoji identifier of the unique gift's model
    pub model_custom_emoji_id: String,

    /// Custom emoji identifier of the unique gift's symbol
    pub symbol_custom_emoji_id: String,

    /// Main color used in light themes; RGB format
    pub light_theme_main_color: i64,

    /// List of 1-3 additional colors used in light themes; RGB format
    pub light_theme_other_colors: Vec<i64>,

    /// Main color used in dark themes; RGB format
    pub dark_theme_main_color: i64,

    /// List of 1-3 additional colors used in dark themes; RGB format
    pub dark_theme_other_colors: Vec<i64>,
}

// Divider: all content below this line will be preserved after code regen
