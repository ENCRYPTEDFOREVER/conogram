use crate::api::API;
use crate::entities::chat_permissions::ChatPermissions;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct RestrictChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub permissions: ChatPermissions,
    #[serde(skip_serializing_if = "is_false", default)]
    pub use_independent_chat_permissions: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

impl_into_future!(RestrictChatMemberRequest<'a>);

///Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass *True* for all permissions to lift restrictions from a user. Returns *True* on success.
#[derive(Clone)]
pub struct RestrictChatMemberRequest<'a> {
    api: &'a API,
    params: RestrictChatMemberParams,
}

impl<'a> RequestT for RestrictChatMemberRequest<'a> {
    type ParamsType = RestrictChatMemberParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "restrictChatMember"
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
impl<'a> RestrictChatMemberRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId, user_id: i64, permissions: ChatPermissions) -> Self {
        Self {
            api,
            params: RestrictChatMemberParams {
                chat_id,
                user_id,
                permissions,
                use_independent_chat_permissions: bool::default(),
                until_date: Option::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier of the target user
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///A JSON-serialized object for new user permissions
    pub fn permissions(mut self, permissions: impl Into<ChatPermissions>) -> Self {
        self.params.permissions = permissions.into();
        self
    }

    ///Pass *True* if chat permissions are set independently. Otherwise, the *can\_send\_other\_messages* and *can\_add\_web\_page\_previews* permissions will imply the *can\_send\_messages*, *can\_send\_audios*, *can\_send\_documents*, *can\_send\_photos*, *can\_send\_videos*, *can\_send\_video\_notes*, and *can\_send\_voice\_notes* permissions; the *can\_send\_polls* permission will imply the *can\_send\_messages* permission.
    pub fn use_independent_chat_permissions(
        mut self,
        use_independent_chat_permissions: impl Into<bool>,
    ) -> Self {
        self.params.use_independent_chat_permissions = use_independent_chat_permissions.into();
        self
    }

    ///Date when restrictions will be lifted for the user, unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    pub fn until_date(mut self, until_date: impl Into<i64>) -> Self {
        self.params.until_date = Some(until_date.into());
        self
    }
}

impl<'a> API {
    ///Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass *True* for all permissions to lift restrictions from a user. Returns *True* on success.
    pub fn restrict_chat_member(
        &'a self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<i64>,
        permissions: impl Into<ChatPermissions>,
    ) -> RestrictChatMemberRequest {
        RestrictChatMemberRequest::new(self, chat_id.into(), user_id.into(), permissions.into())
    }
}

// Divider: all content below this line will be preserved after code regen
