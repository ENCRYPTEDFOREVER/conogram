use conogram_derives::Request;
use serde::Serialize;

use crate::{entities::input_profile_photo::InputProfilePhoto, utils::deserialize_utils::is_false};

/// Changes the profile photo of a managed business account. Requires the *can\_edit\_profile\_photo* business bot right. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setbusinessaccountprofilephoto)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetBusinessAccountProfilePhotoParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// The new profile photo to set
    pub photo: InputProfilePhoto,

    /// Pass True to set the public photo, which will be visible even if the main photo is hidden by the business account's privacy settings. An account can have only one public photo.
    #[serde(skip_serializing_if = "is_false")]
    pub is_public: bool,
}

// Divider: all content below this line will be preserved after code regen
