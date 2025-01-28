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
pub struct HideGeneralForumTopicParams {
    pub chat_id: ChatId,
}

impl_into_future!(HideGeneralForumTopicRequest<'a>);

///Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. The topic will be automatically closed if it was open. Returns *True* on success.
#[derive(Clone)]
pub struct HideGeneralForumTopicRequest<'a> {
    api: &'a Api,
    params: HideGeneralForumTopicParams,
}

impl RequestT for HideGeneralForumTopicRequest<'_> {
    type ParamsType = HideGeneralForumTopicParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "hideGeneralForumTopic"
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
impl<'a> HideGeneralForumTopicRequest<'a> {
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: HideGeneralForumTopicParams {
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

impl Api {
    ///Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. The topic will be automatically closed if it was open. Returns *True* on success.
    pub fn hide_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> HideGeneralForumTopicRequest {
        HideGeneralForumTopicRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
