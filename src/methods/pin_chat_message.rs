use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct PinChatMessageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_id: i64,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_notification: bool,
}

impl_into_future!(PinChatMessageRequest<'a>);

///Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\_pin\_messages' administrator right in a supergroup or 'can\_edit\_messages' administrator right in a channel. Returns *True* on success.
#[derive(Clone)]
pub struct PinChatMessageRequest<'a> {
    api: &'a API,
    params: PinChatMessageParams,
}

impl<'a> RequestT for PinChatMessageRequest<'a> {
    type ParamsType = PinChatMessageParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "pinChatMessage"
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
impl<'a> PinChatMessageRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, message_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: PinChatMessageParams {
                chat_id: chat_id.into(),
                message_id: message_id.into(),
                business_connection_id: Option::default(),
                disable_notification: bool::default(),
            },
        }
    }

    ///Unique identifier of the business connection on behalf of which the message will be pinned
    #[must_use]
    pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(business_connection_id.into());
        self
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Identifier of a message to pin
    #[must_use]
    pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
        self.params.message_id = message_id.into();
        self
    }

    ///Pass *True* if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.
    #[must_use]
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }
}

impl API {
    ///Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\_pin\_messages' administrator right in a supergroup or 'can\_edit\_messages' administrator right in a channel. Returns *True* on success.
    pub fn pin_chat_message(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<i64>,
    ) -> PinChatMessageRequest {
        PinChatMessageRequest::new(self, chat_id, message_id)
    }
}

// Divider: all content below this line will be preserved after code regen
