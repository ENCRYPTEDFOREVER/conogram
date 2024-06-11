use crate::entities::background_type_chat_theme::BackgroundTypeChatTheme;
use crate::entities::background_type_fill::BackgroundTypeFill;
use crate::entities::background_type_pattern::BackgroundTypePattern;
use crate::entities::background_type_wallpaper::BackgroundTypeWallpaper;
use serde::{Deserialize, Serialize};

///This object describes the type of a background. Currently, it can be one of
///
///* [BackgroundTypeFill](https://core.telegram.org/bots/api/#backgroundtypefill)
///* [BackgroundTypeWallpaper](https://core.telegram.org/bots/api/#backgroundtypewallpaper)
///* [BackgroundTypePattern](https://core.telegram.org/bots/api/#backgroundtypepattern)
///* [BackgroundTypeChatTheme](https://core.telegram.org/bots/api/#backgroundtypechattheme)
///API Reference: [link](https://core.telegram.org/bots/api/#backgroundtype)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BackgroundType {
    #[serde(rename = "fill")]
    Fill(BackgroundTypeFill),
    #[serde(rename = "wallpaper")]
    Wallpaper(BackgroundTypeWallpaper),
    #[serde(rename = "pattern")]
    Pattern(BackgroundTypePattern),
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
