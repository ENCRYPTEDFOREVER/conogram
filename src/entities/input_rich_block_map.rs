use serde::Serialize;

use crate::entities::{location::Location, rich_block_caption::RichBlockCaption};

/// A block with a map, corresponding to the custom HTML tag `<tg-map>`. The map's width and height must not exceed 10000 in total. The width and height ratio must be at most 20.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockmap)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "map", tag = "type")]
pub struct InputRichBlockMap {
    /// Location of the center of the map
    pub location: Location,

    /// *Optional*. Map zoom level; 0-24
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<i64>,

    /// *Optional*. Map width; 0-10000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// *Optional*. Map height; 0-10000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// *Optional*. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen
