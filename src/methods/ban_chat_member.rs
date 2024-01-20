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
pub struct BanChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub revoke_messages: bool,
}

impl_into_future!(BanChatMemberRequest<'a>);

///Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless [unbanned](https://core.telegram.org/bots/api/#unbanchatmember) first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
#[derive(Clone)]
pub struct BanChatMemberRequest<'a> {
    api: &'a API,
    params: BanChatMemberParams,
}

impl<'a> RequestT for BanChatMemberRequest<'a> {
    type ParamsType = BanChatMemberParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "banChatMember"
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
impl<'a> BanChatMemberRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: BanChatMemberParams {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
                until_date: Option::default(),
                revoke_messages: bool::default(),
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

    ///Date when the user will be unbanned; Unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.
    pub fn until_date(mut self, until_date: impl Into<i64>) -> Self {
        self.params.until_date = Some(until_date.into());
        self
    }

    ///Pass *True* to delete all messages from the chat for the user that is being removed. If *False*, the user will be able to see messages in the group that were sent before the user was removed. Always *True* for supergroups and channels.
    pub fn revoke_messages(mut self, revoke_messages: impl Into<bool>) -> Self {
        self.params.revoke_messages = revoke_messages.into();
        self
    }
}

impl<'a> API {
    ///Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless [unbanned](https://core.telegram.org/bots/api/#unbanchatmember) first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
    pub fn ban_chat_member(
        &'a self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<i64>,
    ) -> BanChatMemberRequest {
        BanChatMemberRequest::new(self, chat_id.into(), user_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
