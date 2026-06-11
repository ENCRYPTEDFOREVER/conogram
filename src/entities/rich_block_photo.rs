use serde::{Deserialize, Serialize};

use crate::{
    entities::{photo_size::PhotoSize, rich_block_caption::RichBlockCaption},
    utils::deserialize_utils::is_false,
};

/// A block with a photo, corresponding to the HTML tag `<photo>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockphoto)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichBlockPhoto {
    /// Available sizes of the photo
    pub photo: Vec<PhotoSize>,

    /// *Optional*. *True*, if the media preview is covered by a spoiler animation
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_spoiler: bool,

    /// *Optional*. Caption of the block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen
