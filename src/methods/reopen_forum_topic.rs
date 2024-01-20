use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct ReopenForumTopicParams {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
}

impl_into_future!(ReopenForumTopicRequest<'a>);

///Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
#[derive(Clone)]
pub struct ReopenForumTopicRequest<'a> {
    api: &'a API,
    params: ReopenForumTopicParams,
}

impl<'a> RequestT for ReopenForumTopicRequest<'a> {
    type ParamsType = ReopenForumTopicParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "reopenForumTopic"
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
impl<'a> ReopenForumTopicRequest<'a> {
    pub fn new(
        api: &'a API,
        chat_id: impl Into<ChatId>,
        message_thread_id: impl Into<i64>,
    ) -> Self {
        Self {
            api,
            params: ReopenForumTopicParams {
                chat_id: chat_id.into(),
                message_thread_id: message_thread_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier for the target message thread of the forum topic
    pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
        self.params.message_thread_id = message_thread_id.into();
        self
    }
}

impl<'a> API {
    ///Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    pub fn reopen_forum_topic(
        &'a self,
        chat_id: impl Into<ChatId>,
        message_thread_id: impl Into<i64>,
    ) -> ReopenForumTopicRequest {
        ReopenForumTopicRequest::new(self, chat_id.into(), message_thread_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
