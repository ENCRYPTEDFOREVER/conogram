use serde::{Deserialize, Serialize};

///The background is filled using the selected color.
///API Reference: [link](https://core.telegram.org/bots/api/#backgroundfillsolid)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BackgroundFillSolid {
    ///The color of the background fill in the RGB24 format
    pub color: i64,
}
// Divider: all content below this line will be preserved after code regen
