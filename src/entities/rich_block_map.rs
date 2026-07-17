use serde::{Deserialize, Serialize};

use crate::entities::{location::Location, rich_block_caption::RichBlockCaption};

/// A block with a map, corresponding to the custom HTML tag `<tg-map>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockmap)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "map", tag = "type")]
pub struct RichBlockMap {
    /// Location of the center of the map
    pub location: Location,

    /// Map zoom level; 13-20
    pub zoom: i64,

    /// Expected width of the map
    pub width: i64,

    /// Expected height of the map
    pub height: i64,

    /// *Optional*. Caption of the block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen
