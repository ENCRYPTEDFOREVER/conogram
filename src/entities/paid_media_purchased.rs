use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// This object contains information about a paid media purchase.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#paidmediapurchased)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaidMediaPurchased {
    /// User who purchased the media
    pub from: User,

    /// Bot-specified paid media payload
    pub paid_media_payload: String,
}

// Divider: all content below this line will be preserved after code regen
