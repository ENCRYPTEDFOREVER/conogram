use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct CloseForumTopicParams {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
}

impl_into_future!(CloseForumTopicRequest<'a>);

///Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
#[derive(Clone)]
pub struct CloseForumTopicRequest<'a> {
    api: &'a API,
    params: CloseForumTopicParams,
}

impl<'a> RequestT for CloseForumTopicRequest<'a> {
    type ParamsType = CloseForumTopicParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "closeForumTopic"
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
impl<'a> CloseForumTopicRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId, message_thread_id: i64) -> Self {
        Self {
            api,
            params: CloseForumTopicParams {
                chat_id,
                message_thread_id,
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
    ///Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    pub fn close_forum_topic(
        &'a self,
        chat_id: impl Into<ChatId>,
        message_thread_id: impl Into<i64>,
    ) -> CloseForumTopicRequest {
        CloseForumTopicRequest::new(self, chat_id.into(), message_thread_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
