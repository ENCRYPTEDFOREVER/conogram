use conogram_derives::Request;
use serde::Serialize;

use crate::entities::input_profile_photo::InputProfilePhoto;

/// Changes the profile photo of the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setmyprofilephoto)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetMyProfilePhotoParams {
    /// The new profile photo to set
    pub photo: InputProfilePhoto,
}

// Divider: all content below this line will be preserved after code regen
