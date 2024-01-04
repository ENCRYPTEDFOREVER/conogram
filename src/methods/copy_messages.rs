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
pub struct CopyMessagesParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub from_chat_id: ChatId,
    pub message_ids: Vec<i64>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub disable_notification: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub protect_content: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub remove_caption: bool,
}

impl_into_future!(CopyMessagesRequest<'a>);

///Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api/#poll) can be copied only if the value of the field *correct\_option\_id* is known to the bot. The method is analogous to the method [forwardMessages](https://core.telegram.org/bots/api/#forwardmessages), but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
#[derive(Clone)]
pub struct CopyMessagesRequest<'a> {
    api: &'a API,
    params: CopyMessagesParams,
}

impl<'a> RequestT for CopyMessagesRequest<'a> {
    type ParamsType = CopyMessagesParams;
    type ReturnType = Vec<MessageId>;
    fn get_name() -> &'static str {
        "copyMessages"
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
impl<'a> CopyMessagesRequest<'a> {
    pub fn new(
        api: &'a API,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: impl Into<Vec<i64>>,
    ) -> Self {
        Self {
            api,
            params: CopyMessagesParams {
                chat_id: chat_id.into(),
                from_chat_id: from_chat_id.into(),
                message_ids: message_ids.into(),
                message_thread_id: Option::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
                remove_caption: bool::default(),
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

    ///Identifiers of 1-100 messages in the chat *from\_chat\_id* to copy. The identifiers must be specified in a strictly increasing order.
    pub fn message_ids(mut self, message_ids: impl Into<Vec<i64>>) -> Self {
        self.params.message_ids = message_ids.into();
        self
    }

    ///Sends the messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the sent messages from forwarding and saving
    pub fn protect_content(mut self, protect_content: impl Into<bool>) -> Self {
        self.params.protect_content = protect_content.into();
        self
    }

    ///Pass *True* to copy the messages without their captions
    pub fn remove_caption(mut self, remove_caption: impl Into<bool>) -> Self {
        self.params.remove_caption = remove_caption.into();
        self
    }
}

impl<'a> API {
    ///Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api/#poll) can be copied only if the value of the field *correct\_option\_id* is known to the bot. The method is analogous to the method [forwardMessages](https://core.telegram.org/bots/api/#forwardmessages), but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
    pub fn copy_messages(
        &'a self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: impl Into<Vec<i64>>,
    ) -> CopyMessagesRequest {
        CopyMessagesRequest::new(
            self,
            chat_id.into(),
            from_chat_id.into(),
            message_ids.into(),
        )
    }
}

// Divider: all content below this line will be preserved after code regen
