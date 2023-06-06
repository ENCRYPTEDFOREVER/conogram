use crate::api::API;
use crate::entities::message::Message;
use crate::entities::misc::chat_id::ChatId;
use crate::entities::misc::input_file::GetFiles;
use crate::entities::misc::input_file::InputFile;
use crate::entities::misc::input_file::Moose;
use crate::entities::misc::reply_markup::ReplyMarkup;
use crate::errors::Error;
use crate::impl_into_future_multipart;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::collections::HashMap;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SendStickerParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip)]
    pub sticker: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub disable_notification: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub protect_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub allow_sending_without_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl GetFiles for SendStickerParams {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        map.insert(Moose::Owned("sticker".into()), &self.sticker);
        map
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
    pub fn new(api: &'a API, chat_id: ChatId, sticker: InputFile) -> Self {
        Self {
            api,
            params: SendStickerParams {
                chat_id,
                sticker,
                message_thread_id: Option::default(),
                emoji: Option::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
                reply_to_message_id: Option::default(),
                allow_sending_without_reply: bool::default(),
                reply_markup: Option::default(),
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

    ///Sticker to send. Pass a file\_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .WEBP sticker from the Internet, or upload a new .WEBP or .TGS sticker using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Video stickers can only be sent by a file\_id. Animated stickers can't be sent via an HTTP URL.
    pub fn sticker(mut self, sticker: impl Into<InputFile>) -> Self {
        self.params.sticker = sticker.into();
        self
    }

    ///Emoji associated with the sticker; only for just uploaded stickers
    pub fn emoji(mut self, emoji: impl Into<String>) -> Self {
        self.params.emoji = Some(emoji.into());
        self
    }

    ///Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: impl Into<bool>) -> Self {
        self.params.protect_content = protect_content.into();
        self
    }

    ///If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: impl Into<i64>) -> Self {
        self.params.reply_to_message_id = Some(reply_to_message_id.into());
        self
    }

    ///Pass *True* if the message should be sent even if the specified replied-to message is not found
    pub fn allow_sending_without_reply(
        mut self,
        allow_sending_without_reply: impl Into<bool>,
    ) -> Self {
        self.params.allow_sending_without_reply = allow_sending_without_reply.into();
        self
    }

    ///Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl<'a> API {
    ///Use this method to send static .WEBP, [animated](https://telegram.org/blog/animated-stickers) .TGS, or [video](https://telegram.org/blog/video-stickers-better-reactions) .WEBM stickers. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn send_sticker(
        &'a self,
        chat_id: impl Into<ChatId>,
        sticker: impl Into<InputFile>,
    ) -> SendStickerRequest {
        SendStickerRequest::new(self, chat_id.into(), sticker.into())
    }
}

// Divider: all content below this line will be preserved after code regen
