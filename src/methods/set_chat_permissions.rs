use crate::api::API;
use crate::entities::chat_permissions::ChatPermissions;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetChatPermissionsParams {
    pub chat_id: ChatId,
    pub permissions: ChatPermissions,
    #[serde(skip_serializing_if = "is_false", default)]
    pub use_independent_chat_permissions: bool,
}

impl_into_future!(SetChatPermissionsRequest<'a>);

///Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the *can\_restrict\_members* administrator rights. Returns *True* on success.
#[derive(Clone)]
pub struct SetChatPermissionsRequest<'a> {
    api: &'a API,
    params: SetChatPermissionsParams,
}

impl<'a> RequestT for SetChatPermissionsRequest<'a> {
    type ParamsType = SetChatPermissionsParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setChatPermissions"
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
impl<'a> SetChatPermissionsRequest<'a> {
    pub fn new(
        api: &'a API,
        chat_id: impl Into<ChatId>,
        permissions: impl Into<ChatPermissions>,
    ) -> Self {
        Self {
            api,
            params: SetChatPermissionsParams {
                chat_id: chat_id.into(),
                permissions: permissions.into(),
                use_independent_chat_permissions: bool::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///A JSON-serialized object for new default chat permissions
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
}

impl<'a> API {
    ///Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the *can\_restrict\_members* administrator rights. Returns *True* on success.
    pub fn set_chat_permissions(
        &'a self,
        chat_id: impl Into<ChatId>,
        permissions: impl Into<ChatPermissions>,
    ) -> SetChatPermissionsRequest {
        SetChatPermissionsRequest::new(self, chat_id.into(), permissions.into())
    }
}

// Divider: all content below this line will be preserved after code regen
