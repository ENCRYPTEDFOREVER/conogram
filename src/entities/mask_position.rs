use serde::{Deserialize, Serialize};

/// This object describes the position on faces where a mask should be placed by default.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#maskposition)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed. One of “forehead”, “eyes”, “mouth”, or “chin”.
    pub point: MaskPositionPoint,

    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub x_shift: f64,

    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. For example, 1.0 will place the mask just below the default mask position.
    pub y_shift: f64,

    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: f64,
}

/// The part of the face relative to which the mask should be placed. One of “forehead”, “eyes”, “mouth”, or “chin”.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaskPositionPoint {
    /// `forehead`
    #[default]
    #[serde(rename = "forehead")]
    Forehead,

    /// `eyes`
    #[serde(rename = "eyes")]
    Eyes,

    /// `mouth`
    #[serde(rename = "mouth")]
    Mouth,

    /// `chin`
    #[serde(rename = "chin")]
    Chin,
}

// Divider: all content below this line will be preserved after code regen
