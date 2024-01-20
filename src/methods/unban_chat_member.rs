use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct UnbanChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: i64,
    #[serde(default, skip_serializing_if = "is_false")]
    pub only_if_banned: bool,
}

impl_into_future!(UnbanChatMemberRequest<'a>);

///Use this method to unban a previously banned user in a supergroup or channel. The user will **not** return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be **removed** from the chat. If you don't want this, use the parameter *only\_if\_banned*. Returns *True* on success.
#[derive(Clone)]
pub struct UnbanChatMemberRequest<'a> {
    api: &'a API,
    params: UnbanChatMemberParams,
}

impl<'a> RequestT for UnbanChatMemberRequest<'a> {
    type ParamsType = UnbanChatMemberParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "unbanChatMember"
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
impl<'a> UnbanChatMemberRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: UnbanChatMemberParams {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
                only_if_banned: bool::default(),
            },
        }
    }

    ///Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier of the target user
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///Do nothing if the user is not banned
    pub fn only_if_banned(mut self, only_if_banned: impl Into<bool>) -> Self {
        self.params.only_if_banned = only_if_banned.into();
        self
    }
}

impl<'a> API {
    ///Use this method to unban a previously banned user in a supergroup or channel. The user will **not** return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be **removed** from the chat. If you don't want this, use the parameter *only\_if\_banned*. Returns *True* on success.
    pub fn unban_chat_member(
        &'a self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<i64>,
    ) -> UnbanChatMemberRequest {
        UnbanChatMemberRequest::new(self, chat_id.into(), user_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
