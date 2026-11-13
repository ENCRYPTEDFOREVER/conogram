use serde::{Deserialize, Serialize};

/// Describes the position of a rectangle area on a story media or a TON wallet card.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#areaposition)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AreaPosition {
    /// The abscissa of the area's center, as a percentage of the media or the card width
    pub x_percentage: f64,

    /// The ordinate of the area's center, as a percentage of the media or the card height
    pub y_percentage: f64,

    /// The width of the area's rectangle, as a percentage of the media or the card width
    pub width_percentage: f64,

    /// The height of the area's rectangle, as a percentage of the media or the card height
    pub height_percentage: f64,

    /// The clockwise rotation angle of the rectangle, in degrees; 0-360
    pub rotation_angle: f64,

    /// The radius of the rectangle corner rounding, as a percentage of the media or the card width
    pub corner_radius_percentage: f64,
}

// Divider: all content below this line will be preserved after code regen
