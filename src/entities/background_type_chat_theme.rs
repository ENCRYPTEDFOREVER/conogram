use serde::{Deserialize, Serialize};

/// The background is taken directly from a built-in chat theme.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#backgroundtypechattheme)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BackgroundTypeChatTheme {
    /// Name of the chat theme, which is usually an emoji
    pub theme_name: String,
}

// Divider: all content below this line will be preserved after code regen
