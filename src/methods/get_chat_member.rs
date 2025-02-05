


use serde::Serialize;

use crate::{
    api::Api,
    entities::{chat_member::ChatMember, misc::chat_id::ChatId},
    
    impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]

pub struct GetChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: i64,
}

impl_into_future!(GetChatMemberRequest<'a>);

///Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a [ChatMember](https://core.telegram.org/bots/api/#chatmember) object on success.
#[derive(Clone)]
pub struct GetChatMemberRequest<'a> {
    api: &'a Api,
    params: GetChatMemberParams,
}

impl RequestT for GetChatMemberRequest<'_> {
    type ParamsType = GetChatMemberParams;
    type ReturnType = ChatMember;
    fn get_name() -> &'static str {
        "getChatMember"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> GetChatMemberRequest<'a> {
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: GetChatMemberParams {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
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
    ///Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a [ChatMember](https://core.telegram.org/bots/api/#chatmember) object on success.
    pub fn get_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<i64>,
    ) -> GetChatMemberRequest {
        GetChatMemberRequest::new(self, chat_id, user_id)
    }
}

// Divider: all content below this line will be preserved after code regen
