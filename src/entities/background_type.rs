use serde::{Deserialize, Serialize};

use crate::entities::{
    background_type_chat_theme::BackgroundTypeChatTheme, background_type_fill::BackgroundTypeFill,
    background_type_pattern::BackgroundTypePattern,
    background_type_wallpaper::BackgroundTypeWallpaper,
};

/// This object describes the type of a background. Currently, it can be one of
///
/// * [BackgroundTypeFill](https://core.telegram.org/bots/api/#backgroundtypefill)
/// * [BackgroundTypeWallpaper](https://core.telegram.org/bots/api/#backgroundtypewallpaper)
/// * [BackgroundTypePattern](https://core.telegram.org/bots/api/#backgroundtypepattern)
/// * [BackgroundTypeChatTheme](https://core.telegram.org/bots/api/#backgroundtypechattheme)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#backgroundtype)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BackgroundType {
    /// The background is automatically filled based on the selected colors.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#backgroundtypefill)
    #[serde(rename = "fill")]
    Fill(BackgroundTypeFill),

    /// The background is a wallpaper in the JPEG format.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#backgroundtypewallpaper)
    #[serde(rename = "wallpaper")]
    Wallpaper(BackgroundTypeWallpaper),

    /// The background is a PNG or TGV (gzipped subset of SVG with MIME type “application/x-tgwallpattern”) pattern to be combined with the background fill chosen by the user.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#backgroundtypepattern)
    #[serde(rename = "pattern")]
    Pattern(BackgroundTypePattern),

    /// The background is taken directly from a built-in chat theme.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#backgroundtypechattheme)
    #[serde(rename = "chat_theme")]
    ChatTheme(BackgroundTypeChatTheme),
}

impl Default for BackgroundType {
    fn default() -> Self {
        Self::Fill(BackgroundTypeFill::default())
    }
}

impl From<BackgroundTypeFill> for BackgroundType {
    fn from(value: BackgroundTypeFill) -> Self {
        Self::Fill(value)
    }
}

impl From<BackgroundTypeWallpaper> for BackgroundType {
    fn from(value: BackgroundTypeWallpaper) -> Self {
        Self::Wallpaper(value)
    }
}

impl From<BackgroundTypePattern> for BackgroundType {
    fn from(value: BackgroundTypePattern) -> Self {
        Self::Pattern(value)
    }
}

impl From<BackgroundTypeChatTheme> for BackgroundType {
    fn from(value: BackgroundTypeChatTheme) -> Self {
        Self::ChatTheme(value)
    }
}

// Divider: all content below this line will be preserved after code regen
