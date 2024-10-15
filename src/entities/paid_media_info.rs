use serde::{Deserialize, Serialize};

use crate::entities::paid_media::PaidMedia;

/// Describes the paid media added to a message.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#paidmediainfo)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PaidMediaInfo {
    /// The number of Telegram Stars that must be paid to buy access to the media
    pub star_count: i64,

    /// Information about the paid media
    pub paid_media: Vec<PaidMedia>,
}

// Divider: all content below this line will be preserved after code regen
