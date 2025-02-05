use serde::Serialize;

use crate::{api::Api, entities::misc::chat_id::ChatId, impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]

pub struct UnpinAllGeneralForumTopicMessagesParams {
    pub chat_id: ChatId,
}

impl_into_future!(UnpinAllGeneralForumTopicMessagesRequest<'a>);

///Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the *can\_pin\_messages* administrator right in the supergroup. Returns *True* on success.
#[derive(Clone)]
pub struct UnpinAllGeneralForumTopicMessagesRequest<'a> {
    api: &'a Api,
    params: UnpinAllGeneralForumTopicMessagesParams,
}

impl RequestT for UnpinAllGeneralForumTopicMessagesRequest<'_> {
    type ParamsType = UnpinAllGeneralForumTopicMessagesParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "unpinAllGeneralForumTopicMessages"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> UnpinAllGeneralForumTopicMessagesRequest<'a> {
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: UnpinAllGeneralForumTopicMessagesParams {
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
    ///Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the *can\_pin\_messages* administrator right in the supergroup. Returns *True* on success.
    pub fn unpin_all_general_forum_topic_messages(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> UnpinAllGeneralForumTopicMessagesRequest {
        UnpinAllGeneralForumTopicMessagesRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
