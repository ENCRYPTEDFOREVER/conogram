


use serde::Serialize;

use crate::{
    api::Api, entities::misc::chat_id::ChatId,  impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]

pub struct DeclineChatJoinRequestParams {
    pub chat_id: ChatId,
    pub user_id: i64,
}

impl_into_future!(DeclineChatJoinRequestRequest<'a>);

///Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\_invite\_users* administrator right. Returns *True* on success.
#[derive(Clone)]
pub struct DeclineChatJoinRequestRequest<'a> {
    api: &'a Api,
    params: DeclineChatJoinRequestParams,
}

impl RequestT for DeclineChatJoinRequestRequest<'_> {
    type ParamsType = DeclineChatJoinRequestParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "declineChatJoinRequest"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> DeclineChatJoinRequestRequest<'a> {
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: DeclineChatJoinRequestParams {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier of the target user
    #[must_use]
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }
}

impl Api {
    ///Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\_invite\_users* administrator right. Returns *True* on success.
    pub fn decline_chat_join_request(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<i64>,
    ) -> DeclineChatJoinRequestRequest {
        DeclineChatJoinRequestRequest::new(self, chat_id, user_id)
    }
}

// Divider: all content below this line will be preserved after code regen
