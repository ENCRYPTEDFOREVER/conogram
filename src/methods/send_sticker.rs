use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API,
    entities::{
        message::Message,
        misc::{
            chat_id::ChatId,
            input_file::{GetFiles, InputFile},
            reply_markup::ReplyMarkup,
        },
        reply_parameters::ReplyParameters,
    },
    errors::ConogramError,
    impl_into_future_multipart,
    request::RequestT,
    utils::deserialize_utils::is_false,
};

#[derive(Debug, Clone, Serialize)]
pub struct SendStickerParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub sticker: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_notification: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub protect_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl GetFiles for SendStickerParams {
    fn get_files(&self) -> Vec<&InputFile> {
        vec![&self.sticker]
    }
}
impl_into_future_multipart!(SendStickerRequest<'a>);

///Use this method to send static .WEBP, [animated](https://telegram.org/blog/animated-stickers) .TGS, or [video](https://telegram.org/blog/video-stickers-better-reactions) .WEBM stickers. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
#[derive(Clone)]
pub struct SendStickerRequest<'a> {
    api: &'a API,
    params: SendStickerParams,
}

impl<'a> RequestT for SendStickerRequest<'a> {
    type ParamsType = SendStickerParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "sendSticker"
    }
    fn get_api_ref(&self) -> &API {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        true
    }
}
impl<'a> SendStickerRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, sticker: impl Into<InputFile>) -> Self {
        Self {
            api,
            params: SendStickerParams {
                chat_id: chat_id.into(),
                sticker: sticker.into(),
                business_connection_id: Option::default(),
                message_thread_id: Option::default(),
                emoji: Option::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
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

    ///Sticker to send. Pass a file\_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .WEBP sticker from the Internet, or upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Video and animated stickers can't be sent via an HTTP URL.
    #[must_use]
    pub fn sticker(mut self, sticker: impl Into<InputFile>) -> Self {
        self.params.sticker = sticker.into();
        self
    }

    ///Emoji associated with the sticker; only for just uploaded stickers
    #[must_use]
    pub fn emoji(mut self, emoji: impl Into<String>) -> Self {
        self.params.emoji = Some(emoji.into());
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
    ///Use this method to send static .WEBP, [animated](https://telegram.org/blog/animated-stickers) .TGS, or [video](https://telegram.org/blog/video-stickers-better-reactions) .WEBM stickers. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn send_sticker(
        &self,
        chat_id: impl Into<ChatId>,
        sticker: impl Into<InputFile>,
    ) -> SendStickerRequest {
        SendStickerRequest::new(self, chat_id, sticker)
    }
}

// Divider: all content below this line will be preserved after code regen
