


use serde::Serialize;

use crate::{
    api::Api, entities::misc::chat_id::ChatId,  impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]

pub struct GetChatMemberCountParams {
    pub chat_id: ChatId,
}

impl_into_future!(GetChatMemberCountRequest<'a>);

///Use this method to get the number of members in a chat. Returns *Int* on success.
#[derive(Clone)]
pub struct GetChatMemberCountRequest<'a> {
    api: &'a Api,
    params: GetChatMemberCountParams,
}

impl RequestT for GetChatMemberCountRequest<'_> {
    type ParamsType = GetChatMemberCountParams;
    type ReturnType = i64;
    fn get_name() -> &'static str {
        "getChatMemberCount"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> GetChatMemberCountRequest<'a> {
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: GetChatMemberCountParams {
                chat_id: chat_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }
}

impl Api {
    ///Use this method to get the number of members in a chat. Returns *Int* on success.
    pub fn get_chat_member_count(&self, chat_id: impl Into<ChatId>) -> GetChatMemberCountRequest {
        GetChatMemberCountRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
