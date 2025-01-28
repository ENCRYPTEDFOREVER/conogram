use serde::{Deserialize, Serialize};

use crate::entities::photo_size::PhotoSize;

/// This object represent a user's profile pictures.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#userprofilephotos)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: i64,

    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}

// Divider: all content below this line will be preserved after code regen
