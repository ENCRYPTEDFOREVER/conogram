use serde::{Deserialize, Serialize};

use crate::entities::audio::Audio;

/// This object represents the audios displayed on a user's profile.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#userprofileaudios)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfileAudios {
    /// Total number of profile audios for the target user
    pub total_count: i64,

    /// Requested profile audios
    pub audios: Vec<Audio>,
}

// Divider: all content below this line will be preserved after code regen
