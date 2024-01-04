use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct DeleteMessagesParams {
    pub chat_id: ChatId,
    pub message_ids: Vec<i64>,
}

impl_into_future!(DeleteMessagesRequest<'a>);

///Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns *True* on success.
#[derive(Clone)]
pub struct DeleteMessagesRequest<'a> {
    api: &'a API,
    params: DeleteMessagesParams,
}

impl<'a> RequestT for DeleteMessagesRequest<'a> {
    type ParamsType = DeleteMessagesParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "deleteMessages"
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
impl<'a> DeleteMessagesRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, message_ids: impl Into<Vec<i64>>) -> Self {
        Self {
            api,
            params: DeleteMessagesParams {
                chat_id: chat_id.into(),
                message_ids: message_ids.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Identifiers of 1-100 messages to delete. See [deleteMessage](https://core.telegram.org/bots/api/#deletemessage) for limitations on which messages can be deleted
    pub fn message_ids(mut self, message_ids: impl Into<Vec<i64>>) -> Self {
        self.params.message_ids = message_ids.into();
        self
    }
}

impl<'a> API {
    ///Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns *True* on success.
    pub fn delete_messages(
        &'a self,
        chat_id: impl Into<ChatId>,
        message_ids: impl Into<Vec<i64>>,
    ) -> DeleteMessagesRequest {
        DeleteMessagesRequest::new(self, chat_id.into(), message_ids.into())
    }
}

// Divider: all content below this line will be preserved after code regen
