use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API, entities::misc::chat_id::ChatId, errors::ConogramError, impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct UnhideGeneralForumTopicParams {
    pub chat_id: ChatId,
}

impl_into_future!(UnhideGeneralForumTopicRequest<'a>);

///Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. Returns *True* on success.
#[derive(Clone)]
pub struct UnhideGeneralForumTopicRequest<'a> {
    api: &'a API,
    params: UnhideGeneralForumTopicParams,
}

impl<'a> RequestT for UnhideGeneralForumTopicRequest<'a> {
    type ParamsType = UnhideGeneralForumTopicParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "unhideGeneralForumTopic"
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
impl<'a> UnhideGeneralForumTopicRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: UnhideGeneralForumTopicParams {
                chat_id: chat_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }
}

impl API {
    ///Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. Returns *True* on success.
    pub fn unhide_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> UnhideGeneralForumTopicRequest {
        UnhideGeneralForumTopicRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
