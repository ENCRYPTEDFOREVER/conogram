use crate::entities::background_fill::BackgroundFill;
use serde::{Deserialize, Serialize};

///The background is automatically filled based on the selected colors.
///API Reference: [link](https://core.telegram.org/bots/api/#backgroundtypefill)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BackgroundTypeFill {
    ///The background fill
    pub fill: BackgroundFill,

    ///Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: i64,
}
// Divider: all content below this line will be preserved after code regen
