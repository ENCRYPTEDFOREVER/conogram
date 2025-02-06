use conogram_derives::Request;
use serde::Serialize;

use crate::entities::user_profile_photos::UserProfilePhotos;

/// Use this method to get a list of profile pictures for a user. Returns a [UserProfilePhotos](https://core.telegram.org/bots/api/#userprofilephotos) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getuserprofilephotos)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = UserProfilePhotos)]
pub struct GetUserProfilePhotosParams {
    /// Unique identifier of the target user
    pub user_id: i64,

    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
