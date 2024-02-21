use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct EditGeneralForumTopicParams {
    pub chat_id: ChatId,
    pub name: String,
}

impl_into_future!(EditGeneralForumTopicRequest<'a>);

///Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have *can\_manage\_topics* administrator rights. Returns *True* on success.
#[derive(Clone)]
pub struct EditGeneralForumTopicRequest<'a> {
    api: &'a API,
    params: EditGeneralForumTopicParams,
}

impl<'a> RequestT for EditGeneralForumTopicRequest<'a> {
    type ParamsType = EditGeneralForumTopicParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "editGeneralForumTopic"
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
impl<'a> EditGeneralForumTopicRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, name: impl Into<String>) -> Self {
        Self {
            api,
            params: EditGeneralForumTopicParams {
                chat_id: chat_id.into(),
                name: name.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///New topic name, 1-128 characters
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = name.into();
        self
    }
}

impl<'a> API {
    ///Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have *can\_manage\_topics* administrator rights. Returns *True* on success.
    pub fn edit_general_forum_topic(
        &'a self,
        chat_id: impl Into<ChatId>,
        name: impl Into<String>,
    ) -> EditGeneralForumTopicRequest {
        EditGeneralForumTopicRequest::new(self, chat_id, name)
    }
}

// Divider: all content below this line will be preserved after code regen
