use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct UnpinAllForumTopicMessagesParams {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
}

impl_into_future!(UnpinAllForumTopicMessagesRequest<'a>);

///Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the *can\_pin\_messages* administrator right in the supergroup. Returns *True* on success.
#[derive(Clone)]
pub struct UnpinAllForumTopicMessagesRequest<'a> {
    api: &'a API,
    params: UnpinAllForumTopicMessagesParams,
}

impl<'a> RequestT for UnpinAllForumTopicMessagesRequest<'a> {
    type ParamsType = UnpinAllForumTopicMessagesParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "unpinAllForumTopicMessages"
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
impl<'a> UnpinAllForumTopicMessagesRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId, message_thread_id: i64) -> Self {
        Self {
            api,
            params: UnpinAllForumTopicMessagesParams {
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
    ///Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the *can\_pin\_messages* administrator right in the supergroup. Returns *True* on success.
    pub fn unpin_all_forum_topic_messages(
        &'a self,
        chat_id: impl Into<ChatId>,
        message_thread_id: impl Into<i64>,
    ) -> UnpinAllForumTopicMessagesRequest {
        UnpinAllForumTopicMessagesRequest::new(self, chat_id.into(), message_thread_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
