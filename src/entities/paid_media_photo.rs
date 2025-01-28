use serde::{Deserialize, Serialize};

use crate::entities::photo_size::PhotoSize;

/// The paid media is a photo.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#paidmediaphoto)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaidMediaPhoto {
    /// The photo
    pub photo: Vec<PhotoSize>,
}

// Divider: all content below this line will be preserved after code regen
