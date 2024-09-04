use crate::entities::document::Document;
use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///The background is a wallpaper in the JPEG format.
///
///API Reference: [link](https://core.telegram.org/bots/api/#backgroundtypewallpaper)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BackgroundTypeWallpaper {
    ///Document with the wallpaper
    pub document: Document,

    ///Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: i64,

    ///*Optional*. *True*, if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_blurred: bool,

    ///*Optional*. *True*, if the background moves slightly when the device is tilted
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_moving: bool,
}
// Divider: all content below this line will be preserved after code regen
