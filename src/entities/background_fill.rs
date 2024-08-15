use crate::entities::background_fill_freeform_gradient::BackgroundFillFreeformGradient;
use crate::entities::background_fill_gradient::BackgroundFillGradient;
use crate::entities::background_fill_solid::BackgroundFillSolid;
use serde::{Deserialize, Serialize};

///This object describes the way a background is filled based on the selected colors. Currently, it can be one of
///
///* [BackgroundFillSolid](https://core.telegram.org/bots/api/#backgroundfillsolid)
///* [BackgroundFillGradient](https://core.telegram.org/bots/api/#backgroundfillgradient)
///* [BackgroundFillFreeformGradient](https://core.telegram.org/bots/api/#backgroundfillfreeformgradient)
///
///API Reference: [link](https://core.telegram.org/bots/api/#backgroundfill)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BackgroundFill {
    #[serde(rename = "solid")]
    Solid(BackgroundFillSolid),
    #[serde(rename = "gradient")]
    Gradient(BackgroundFillGradient),
    #[serde(rename = "freeform_gradient")]
    FreeformGradient(BackgroundFillFreeformGradient),
}
impl Default for BackgroundFill {
    fn default() -> Self {
        Self::Solid(BackgroundFillSolid::default())
    }
}
impl From<BackgroundFillSolid> for BackgroundFill {
    fn from(value: BackgroundFillSolid) -> Self {
        Self::Solid(value)
    }
}

impl From<BackgroundFillGradient> for BackgroundFill {
    fn from(value: BackgroundFillGradient) -> Self {
        Self::Gradient(value)
    }
}

impl From<BackgroundFillFreeformGradient> for BackgroundFill {
    fn from(value: BackgroundFillFreeformGradient) -> Self {
        Self::FreeformGradient(value)
    }
}
// Divider: all content below this line will be preserved after code regen
