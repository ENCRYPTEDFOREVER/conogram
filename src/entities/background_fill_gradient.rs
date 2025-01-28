use serde::{Deserialize, Serialize};

/// The background is a gradient fill.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#backgroundfillgradient)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackgroundFillGradient {
    /// Top color of the gradient in the RGB24 format
    pub top_color: i64,

    /// Bottom color of the gradient in the RGB24 format
    pub bottom_color: i64,

    /// Clockwise rotation angle of the background fill in degrees; 0-359
    pub rotation_angle: i64,
}

// Divider: all content below this line will be preserved after code regen
