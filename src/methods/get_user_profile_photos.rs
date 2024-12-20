use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API, entities::user_profile_photos::UserProfilePhotos, errors::ConogramError,
    impl_into_future, request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetUserProfilePhotosParams {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl_into_future!(GetUserProfilePhotosRequest<'a>);

///Use this method to get a list of profile pictures for a user. Returns a [UserProfilePhotos](https://core.telegram.org/bots/api/#userprofilephotos) object.
#[derive(Clone)]
pub struct GetUserProfilePhotosRequest<'a> {
    api: &'a API,
    params: GetUserProfilePhotosParams,
}

impl<'a> RequestT for GetUserProfilePhotosRequest<'a> {
    type ParamsType = GetUserProfilePhotosParams;
    type ReturnType = UserProfilePhotos;
    fn get_name() -> &'static str {
        "getUserProfilePhotos"
    }
    fn get_api_ref(&self) -> &API {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        false
    }
}
impl<'a> GetUserProfilePhotosRequest<'a> {
    pub fn new(api: &'a API, user_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: GetUserProfilePhotosParams {
                user_id: user_id.into(),
                offset: Option::default(),
                limit: Option::default(),
            },
        }
    }

    ///Unique identifier of the target user
    #[must_use]
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///Sequential number of the first photo to be returned. By default, all photos are returned.
    #[must_use]
    pub fn offset(mut self, offset: impl Into<i64>) -> Self {
        self.params.offset = Some(offset.into());
        self
    }

    ///Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[must_use]
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.params.limit = Some(limit.into());
        self
    }
}

impl API {
    ///Use this method to get a list of profile pictures for a user. Returns a [UserProfilePhotos](https://core.telegram.org/bots/api/#userprofilephotos) object.
    pub fn get_user_profile_photos(&self, user_id: impl Into<i64>) -> GetUserProfilePhotosRequest {
        GetUserProfilePhotosRequest::new(self, user_id)
    }
}

// Divider: all content below this line will be preserved after code regen
