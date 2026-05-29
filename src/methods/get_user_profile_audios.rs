use conogram_derives::Request;
use serde::Serialize;

use crate::entities::user_profile_audios::UserProfileAudios;

/// Use this method to get a list of profile audios for a user. Returns a [UserProfileAudios](https://core.telegram.org/bots/api/#userprofileaudios) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getuserprofileaudios)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = UserProfileAudios)]
pub struct GetUserProfileAudiosParams {
    /// Unique identifier of the target user
    pub user_id: i64,

    /// Sequential number of the first audio to be returned. By default, all audios are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Limits the number of audios to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
