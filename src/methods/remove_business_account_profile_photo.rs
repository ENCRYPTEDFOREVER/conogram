use conogram_derives::Request;
use serde::Serialize;

use crate::utils::deserialize_utils::is_false;

/// Removes the current profile photo of a managed business account. Requires the *can\_edit\_profile\_photo* business bot right. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#removebusinessaccountprofilephoto)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct RemoveBusinessAccountProfilePhotoParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// Pass True to remove the public photo, which is visible even if the main photo is hidden by the business account's privacy settings. After the main photo is removed, the previous profile photo (if present) becomes the main photo.
    #[serde(skip_serializing_if = "is_false")]
    pub is_public: bool,
}

// Divider: all content below this line will be preserved after code regen
