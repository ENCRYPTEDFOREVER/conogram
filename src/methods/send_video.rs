use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API,
    entities::{
        message::Message,
        message_entity::MessageEntity,
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
pub struct SendVideoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub video: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub show_caption_above_media: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_spoiler: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub supports_streaming: bool,
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

impl GetFiles for SendVideoParams {
    fn get_files(&self) -> Vec<&InputFile> {
        let mut vec = Vec::with_capacity(4);
        vec.push(&self.video);
        if let Some(thumbnail) = &self.thumbnail {
            vec.push(thumbnail);
        }
        vec
    }
}
impl_into_future_multipart!(SendVideoRequest<'a>);

///Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as [Document](https://core.telegram.org/bots/api/#document)). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Clone)]
pub struct SendVideoRequest<'a> {
    api: &'a API,
    params: SendVideoParams,
}

impl<'a> RequestT for SendVideoRequest<'a> {
    type ParamsType = SendVideoParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "sendVideo"
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
impl<'a> SendVideoRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, video: impl Into<InputFile>) -> Self {
        Self {
            api,
            params: SendVideoParams {
                chat_id: chat_id.into(),
                video: video.into(),
                business_connection_id: Option::default(),
                message_thread_id: Option::default(),
                duration: Option::default(),
                width: Option::default(),
                height: Option::default(),
                thumbnail: Option::default(),
                caption: Option::default(),
                parse_mode: Option::default(),
                caption_entities: Vec::default(),
                show_caption_above_media: bool::default(),
                has_spoiler: bool::default(),
                supports_streaming: bool::default(),
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

    ///Video to send. Pass a file\_id as String to send a video that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet, or upload a new video using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    #[must_use]
    pub fn video(mut self, video: impl Into<InputFile>) -> Self {
        self.params.video = video.into();
        self
    }

    ///Duration of sent video in seconds
    #[must_use]
    pub fn duration(mut self, duration: impl Into<i64>) -> Self {
        self.params.duration = Some(duration.into());
        self
    }

    ///Video width
    #[must_use]
    pub fn width(mut self, width: impl Into<i64>) -> Self {
        self.params.width = Some(width.into());
        self
    }

    ///Video height
    #[must_use]
    pub fn height(mut self, height: impl Into<i64>) -> Self {
        self.params.height = Some(height.into());
        self
    }

    ///Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    #[must_use]
    pub fn thumbnail(mut self, thumbnail: impl Into<InputFile>) -> Self {
        self.params.thumbnail = Some(thumbnail.into());
        self
    }

    ///Video caption (may also be used when resending videos by *file\_id*), 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.params.caption = Some(caption.into());
        self
    }

    ///Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[must_use]
    pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
        self.params.parse_mode = Some(parse_mode.into());
        self
    }

    ///A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[must_use]
    pub fn caption_entities(
        mut self,
        caption_entities: impl IntoIterator<Item = impl Into<MessageEntity>>,
    ) -> Self {
        self.params.caption_entities = caption_entities.into_iter().map(Into::into).collect();
        self
    }

    ///Pass *True*, if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media(mut self, show_caption_above_media: impl Into<bool>) -> Self {
        self.params.show_caption_above_media = show_caption_above_media.into();
        self
    }

    ///Pass *True* if the video needs to be covered with a spoiler animation
    #[must_use]
    pub fn has_spoiler(mut self, has_spoiler: impl Into<bool>) -> Self {
        self.params.has_spoiler = has_spoiler.into();
        self
    }

    ///Pass *True* if the uploaded video is suitable for streaming
    #[must_use]
    pub fn supports_streaming(mut self, supports_streaming: impl Into<bool>) -> Self {
        self.params.supports_streaming = supports_streaming.into();
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
    ///Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as [Document](https://core.telegram.org/bots/api/#document)). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
    pub fn send_video(
        &self,
        chat_id: impl Into<ChatId>,
        video: impl Into<InputFile>,
    ) -> SendVideoRequest {
        SendVideoRequest::new(self, chat_id, video)
    }
}

// Divider: all content below this line will be preserved after code regen
