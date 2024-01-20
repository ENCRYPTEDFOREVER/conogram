use crate::api::API;
use crate::entities::message_id::MessageId;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct ForwardMessagesParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub from_chat_id: ChatId,
    pub message_ids: Vec<i64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_notification: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub protect_content: bool,
}

impl_into_future!(ForwardMessagesRequest<'a>);

///Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
#[derive(Clone)]
pub struct ForwardMessagesRequest<'a> {
    api: &'a API,
    params: ForwardMessagesParams,
}

impl<'a> RequestT for ForwardMessagesRequest<'a> {
    type ParamsType = ForwardMessagesParams;
    type ReturnType = Vec<MessageId>;
    fn get_name() -> &'static str {
        "forwardMessages"
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
impl<'a> ForwardMessagesRequest<'a> {
    pub fn new(
        api: &'a API,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: impl Into<Vec<i64>>,
    ) -> Self {
        Self {
            api,
            params: ForwardMessagesParams {
                chat_id: chat_id.into(),
                from_chat_id: from_chat_id.into(),
                message_ids: message_ids.into(),
                message_thread_id: Option::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
        self.params.message_thread_id = Some(message_thread_id.into());
        self
    }

    ///Unique identifier for the chat where the original messages were sent (or channel username in the format `@channelusername`)
    pub fn from_chat_id(mut self, from_chat_id: impl Into<ChatId>) -> Self {
        self.params.from_chat_id = from_chat_id.into();
        self
    }

    ///Identifiers of 1-100 messages in the chat *from\_chat\_id* to forward. The identifiers must be specified in a strictly increasing order.
    pub fn message_ids(mut self, message_ids: impl IntoIterator<Item = impl Into<i64>>) -> Self {
        self.params.message_ids = message_ids.into_iter().map(Into::into).collect();
        self
    }

    ///Sends the messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the forwarded messages from forwarding and saving
    pub fn protect_content(mut self, protect_content: impl Into<bool>) -> Self {
        self.params.protect_content = protect_content.into();
        self
    }
}

impl<'a> API {
    ///Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
    pub fn forward_messages(
        &'a self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: impl Into<Vec<i64>>,
    ) -> ForwardMessagesRequest {
        ForwardMessagesRequest::new(
            self,
            chat_id.into(),
            from_chat_id.into(),
            message_ids.into(),
        )
    }
}

// Divider: all content below this line will be preserved after code regen
