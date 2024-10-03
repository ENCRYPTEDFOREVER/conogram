use serde::{Deserialize, Serialize};

use crate::entities::{
    background_fill_freeform_gradient::BackgroundFillFreeformGradient,
    background_fill_gradient::BackgroundFillGradient, background_fill_solid::BackgroundFillSolid,
};

/// This object describes the way a background is filled based on the selected colors. Currently, it can be one of
///
/// * [BackgroundFillSolid](https://core.telegram.org/bots/api/#backgroundfillsolid)
/// * [BackgroundFillGradient](https://core.telegram.org/bots/api/#backgroundfillgradient)
/// * [BackgroundFillFreeformGradient](https://core.telegram.org/bots/api/#backgroundfillfreeformgradient)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#backgroundfill)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BackgroundFill {
    /// The background is filled using the selected color.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#backgroundfillsolid)
    #[serde(rename = "solid")]
    Solid(BackgroundFillSolid),

    /// The background is a gradient fill.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#backgroundfillgradient)
    #[serde(rename = "gradient")]
    Gradient(BackgroundFillGradient),

    /// The background is a freeform gradient that rotates after every message in the chat.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#backgroundfillfreeformgradient)
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
