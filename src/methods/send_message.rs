use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API,
    entities::{
        link_preview_options::LinkPreviewOptions,
        message::Message,
        message_entity::MessageEntity,
        misc::{chat_id::ChatId, reply_markup::ReplyMarkup},
        reply_parameters::ReplyParameters,
    },
    errors::ConogramError,
    impl_into_future,
    request::RequestT,
    utils::deserialize_utils::is_false,
};

#[derive(Debug, Clone, Serialize)]
pub struct SendMessageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<MessageEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_notification: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub protect_content: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_paid_broadcast: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl_into_future!(SendMessageRequest<'a>);

///Use this method to send text messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
#[derive(Clone)]
pub struct SendMessageRequest<'a> {
    api: &'a API,
    params: SendMessageParams,
}

impl<'a> RequestT for SendMessageRequest<'a> {
    type ParamsType = SendMessageParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "sendMessage"
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
impl<'a> SendMessageRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, text: impl Into<String>) -> Self {
        Self {
            api,
            params: SendMessageParams {
                chat_id: chat_id.into(),
                text: text.into(),
                business_connection_id: Option::default(),
                message_thread_id: Option::default(),
                parse_mode: Option::default(),
                entities: Vec::default(),
                link_preview_options: Option::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
                allow_paid_broadcast: bool::default(),
                message_effect_id: Option::default(),
                reply_parameters: Option::default(),
                reply_markup: Option::default(),
            },
        }
    }

    ///Unique identifier of the business connection on behalf of which the message will be sent
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

    ///Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[must_use]
    pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
        self.params.message_thread_id = Some(message_thread_id.into());
        self
    }

    ///Text of the message to be sent, 1-4096 characters after entities parsing
    #[must_use]
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.params.text = text.into();
        self
    }

    ///Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[must_use]
    pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
        self.params.parse_mode = Some(parse_mode.into());
        self
    }

    ///A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse\_mode*
    #[must_use]
    pub fn entities(
        mut self,
        entities: impl IntoIterator<Item = impl Into<MessageEntity>>,
    ) -> Self {
        self.params.entities = entities.into_iter().map(Into::into).collect();
        self
    }

    ///Link preview generation options for the message
    #[must_use]
    pub fn link_preview_options(
        mut self,
        link_preview_options: impl Into<LinkPreviewOptions>,
    ) -> Self {
        self.params.link_preview_options = Some(link_preview_options.into());
        self
    }

    ///Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content(mut self, protect_content: impl Into<bool>) -> Self {
        self.params.protect_content = protect_content.into();
        self
    }

    ///Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[must_use]
    pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: impl Into<bool>) -> Self {
        self.params.allow_paid_broadcast = allow_paid_broadcast.into();
        self
    }

    ///Unique identifier of the message effect to be added to the message; for private chats only
    #[must_use]
    pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(message_effect_id.into());
        self
    }

    ///Description of the message to reply to
    #[must_use]
    pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
        self.params.reply_parameters = Some(reply_parameters.into());
        self
    }

    ///Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
    #[must_use]
    pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl API {
    ///Use this method to send text messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn send_message(
        &self,
        chat_id: impl Into<ChatId>,
        text: impl Into<String>,
    ) -> SendMessageRequest {
        SendMessageRequest::new(self, chat_id, text)
    }
}

// Divider: all content below this line will be preserved after code regen

impl SendMessageRequest<'_> {
    /// For backwards compatibility (Bot API < 7.0)
    #[must_use]
    pub fn disable_web_page_preview(self, disable: bool) -> Self {
        self.link_preview_options(LinkPreviewOptions {
            is_disabled: disable,
            ..Default::default()
        })
    }
}
