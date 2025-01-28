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
pub struct DeleteMessagesParams {
    pub chat_id: ChatId,
    pub message_ids: Vec<i64>,
}

impl_into_future!(DeleteMessagesRequest<'a>);

///Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns *True* on success.
#[derive(Clone)]
pub struct DeleteMessagesRequest<'a> {
    api: &'a Api,
    params: DeleteMessagesParams,
}

impl RequestT for DeleteMessagesRequest<'_> {
    type ParamsType = DeleteMessagesParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "deleteMessages"
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
impl<'a> DeleteMessagesRequest<'a> {
    pub fn new(
        api: &'a Api,
        chat_id: impl Into<ChatId>,
        message_ids: impl IntoIterator<Item = impl Into<i64>>,
    ) -> Self {
        Self {
            api,
            params: DeleteMessagesParams {
                chat_id: chat_id.into(),
                message_ids: message_ids.into_iter().map(Into::into).collect(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///A JSON-serialized list of 1-100 identifiers of messages to delete. See [deleteMessage](https://core.telegram.org/bots/api/#deletemessage) for limitations on which messages can be deleted
    #[must_use]
    pub fn message_ids(mut self, message_ids: impl IntoIterator<Item = impl Into<i64>>) -> Self {
        self.params.message_ids = message_ids.into_iter().map(Into::into).collect();
        self
    }
}

impl Api {
    ///Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns *True* on success.
    pub fn delete_messages(
        &self,
        chat_id: impl Into<ChatId>,
        message_ids: impl IntoIterator<Item = impl Into<i64>>,
    ) -> DeleteMessagesRequest {
        DeleteMessagesRequest::new(self, chat_id, message_ids)
    }
}

// Divider: all content below this line will be preserved after code regen
