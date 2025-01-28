use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{api::Api, errors::ConogramError, impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]
pub struct SetUserEmojiStatusParams {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,
}

impl_into_future!(SetUserEmojiStatusRequest<'a>);

///Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method [requestEmojiStatusAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps). Returns *True* on success.
#[derive(Clone)]
pub struct SetUserEmojiStatusRequest<'a> {
    api: &'a Api,
    params: SetUserEmojiStatusParams,
}

impl RequestT for SetUserEmojiStatusRequest<'_> {
    type ParamsType = SetUserEmojiStatusParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setUserEmojiStatus"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        false
    }
}
impl<'a> SetUserEmojiStatusRequest<'a> {
    pub fn new(api: &'a Api, user_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: SetUserEmojiStatusParams {
                user_id: user_id.into(),
                emoji_status_custom_emoji_id: Option::default(),
                emoji_status_expiration_date: Option::default(),
            },
        }
    }

    ///Unique identifier of the target user
    #[must_use]
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///Custom emoji identifier of the emoji status to set. Pass an empty string to remove the status.
    #[must_use]
    pub fn emoji_status_custom_emoji_id(
        mut self,
        emoji_status_custom_emoji_id: impl Into<String>,
    ) -> Self {
        self.params.emoji_status_custom_emoji_id = Some(emoji_status_custom_emoji_id.into());
        self
    }

    ///Expiration date of the emoji status, if any
    #[must_use]
    pub fn emoji_status_expiration_date(
        mut self,
        emoji_status_expiration_date: impl Into<i64>,
    ) -> Self {
        self.params.emoji_status_expiration_date = Some(emoji_status_expiration_date.into());
        self
    }
}

impl Api {
    ///Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method [requestEmojiStatusAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps). Returns *True* on success.
    pub fn set_user_emoji_status(&self, user_id: impl Into<i64>) -> SetUserEmojiStatusRequest {
        SetUserEmojiStatusRequest::new(self, user_id)
    }
}

// Divider: all content below this line will be preserved after code regen
