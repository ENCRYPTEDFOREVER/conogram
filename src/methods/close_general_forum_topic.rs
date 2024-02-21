use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct CloseGeneralForumTopicParams {
    pub chat_id: ChatId,
}

impl_into_future!(CloseGeneralForumTopicRequest<'a>);

///Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. Returns *True* on success.
#[derive(Clone)]
pub struct CloseGeneralForumTopicRequest<'a> {
    api: &'a API,
    params: CloseGeneralForumTopicParams,
}

impl<'a> RequestT for CloseGeneralForumTopicRequest<'a> {
    type ParamsType = CloseGeneralForumTopicParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "closeGeneralForumTopic"
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
impl<'a> CloseGeneralForumTopicRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: CloseGeneralForumTopicParams {
                chat_id: chat_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }
}

impl<'a> API {
    ///Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. Returns *True* on success.
    pub fn close_general_forum_topic(
        &'a self,
        chat_id: impl Into<ChatId>,
    ) -> CloseGeneralForumTopicRequest {
        CloseGeneralForumTopicRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
