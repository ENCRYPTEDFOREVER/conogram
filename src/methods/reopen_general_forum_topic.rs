use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct ReopenGeneralForumTopicParams {
    pub chat_id: ChatId,
}

impl_into_future!(ReopenGeneralForumTopicRequest<'a>);

///Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. The topic will be automatically unhidden if it was hidden. Returns *True* on success.
#[derive(Clone)]
pub struct ReopenGeneralForumTopicRequest<'a> {
    api: &'a API,
    params: ReopenGeneralForumTopicParams,
}

impl<'a> RequestT for ReopenGeneralForumTopicRequest<'a> {
    type ParamsType = ReopenGeneralForumTopicParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "reopenGeneralForumTopic"
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
impl<'a> ReopenGeneralForumTopicRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId) -> Self {
        Self {
            api,
            params: ReopenGeneralForumTopicParams { chat_id },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }
}

impl<'a> API {
    ///Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. The topic will be automatically unhidden if it was hidden. Returns *True* on success.
    pub fn reopen_general_forum_topic(
        &'a self,
        chat_id: impl Into<ChatId>,
    ) -> ReopenGeneralForumTopicRequest {
        ReopenGeneralForumTopicRequest::new(self, chat_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
