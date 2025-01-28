use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api, entities::misc::chat_id::ChatId, errors::ConogramError, impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct CloseForumTopicParams {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
}

impl_into_future!(CloseForumTopicRequest<'a>);

///Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
#[derive(Clone)]
pub struct CloseForumTopicRequest<'a> {
    api: &'a Api,
    params: CloseForumTopicParams,
}

impl RequestT for CloseForumTopicRequest<'_> {
    type ParamsType = CloseForumTopicParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "closeForumTopic"
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
impl<'a> CloseForumTopicRequest<'a> {
    pub fn new(
        api: &'a Api,
        chat_id: impl Into<ChatId>,
        message_thread_id: impl Into<i64>,
    ) -> Self {
        Self {
            api,
            params: CloseForumTopicParams {
                chat_id: chat_id.into(),
                message_thread_id: message_thread_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier for the target message thread of the forum topic
    #[must_use]
    pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
        self.params.message_thread_id = message_thread_id.into();
        self
    }
}

impl Api {
    ///Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    pub fn close_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: impl Into<i64>,
    ) -> CloseForumTopicRequest {
        CloseForumTopicRequest::new(self, chat_id, message_thread_id)
    }
}

// Divider: all content below this line will be preserved after code regen
