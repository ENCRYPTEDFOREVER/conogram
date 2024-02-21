use crate::api::API;
use crate::entities::message::Message;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct ForwardMessageParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub from_chat_id: ChatId,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_notification: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub protect_content: bool,
    pub message_id: i64,
}

impl_into_future!(ForwardMessageRequest<'a>);

///Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
#[derive(Clone)]
pub struct ForwardMessageRequest<'a> {
    api: &'a API,
    params: ForwardMessageParams,
}

impl<'a> RequestT for ForwardMessageRequest<'a> {
    type ParamsType = ForwardMessageParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "forwardMessage"
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
impl<'a> ForwardMessageRequest<'a> {
    pub fn new(
        api: &'a API,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: impl Into<i64>,
    ) -> Self {
        Self {
            api,
            params: ForwardMessageParams {
                chat_id: chat_id.into(),
                from_chat_id: from_chat_id.into(),
                message_id: message_id.into(),
                message_thread_id: Option::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[must_use]
    pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
        self.params.message_thread_id = Some(message_thread_id.into());
        self
    }

    ///Unique identifier for the chat where the original message was sent (or channel username in the format `@channelusername`)
    #[must_use]
    pub fn from_chat_id(mut self, from_chat_id: impl Into<ChatId>) -> Self {
        self.params.from_chat_id = from_chat_id.into();
        self
    }

    ///Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the forwarded message from forwarding and saving
    #[must_use]
    pub fn protect_content(mut self, protect_content: impl Into<bool>) -> Self {
        self.params.protect_content = protect_content.into();
        self
    }

    ///Message identifier in the chat specified in *from\_chat\_id*
    #[must_use]
    pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
        self.params.message_id = message_id.into();
        self
    }
}

impl<'a> API {
    ///Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn forward_message(
        &'a self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: impl Into<i64>,
    ) -> ForwardMessageRequest {
        ForwardMessageRequest::new(self, chat_id, from_chat_id, message_id)
    }
}

// Divider: all content below this line will be preserved after code regen
