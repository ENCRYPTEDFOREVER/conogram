use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct DeleteForumTopicParams {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
}

impl_into_future!(DeleteForumTopicRequest<'a>);

///Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_delete\_messages* administrator rights. Returns *True* on success.
#[derive(Clone)]
pub struct DeleteForumTopicRequest<'a> {
    api: &'a API,
    params: DeleteForumTopicParams,
}

impl<'a> RequestT for DeleteForumTopicRequest<'a> {
    type ParamsType = DeleteForumTopicParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "deleteForumTopic"
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
impl<'a> DeleteForumTopicRequest<'a> {
    pub fn new(
        api: &'a API,
        chat_id: impl Into<ChatId>,
        message_thread_id: impl Into<i64>,
    ) -> Self {
        Self {
            api,
            params: DeleteForumTopicParams {
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
    ///Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_delete\_messages* administrator rights. Returns *True* on success.
    pub fn delete_forum_topic(
        &'a self,
        chat_id: impl Into<ChatId>,
        message_thread_id: impl Into<i64>,
    ) -> DeleteForumTopicRequest {
        DeleteForumTopicRequest::new(self, chat_id, message_thread_id)
    }
}

// Divider: all content below this line will be preserved after code regen
