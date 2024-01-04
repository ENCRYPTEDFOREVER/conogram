use crate::api::API;
use crate::entities::message::Message;
use crate::entities::message_entity::MessageEntity;
use crate::entities::misc::chat_id::ChatId;
use crate::entities::misc::input_file::GetFiles;
use crate::entities::misc::input_file::InputFile;
use crate::entities::misc::input_file::Moose;
use crate::entities::misc::reply_markup::ReplyMarkup;
use crate::entities::reply_parameters::ReplyParameters;
use crate::errors::ConogramError;
use crate::impl_into_future_multipart;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::collections::HashMap;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SendAnimationParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip)]
    pub animation: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", skip)]
    pub thumbnail: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_spoiler: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub disable_notification: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub protect_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl GetFiles for SendAnimationParams {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        map.insert(Moose::Owned("animation".into()), &self.animation);
        if let Some(thumbnail) = &self.thumbnail {
            map.insert(Moose::Owned("thumbnail".into()), thumbnail);
        }
        map
    }
}
impl_into_future_multipart!(SendAnimationRequest<'a>);

///Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Clone)]
pub struct SendAnimationRequest<'a> {
    api: &'a API,
    params: SendAnimationParams,
}

impl<'a> RequestT for SendAnimationRequest<'a> {
    type ParamsType = SendAnimationParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "sendAnimation"
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
impl<'a> SendAnimationRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, animation: impl Into<InputFile>) -> Self {
        Self {
            api,
            params: SendAnimationParams {
                chat_id: chat_id.into(),
                animation: animation.into(),
                message_thread_id: Option::default(),
                duration: Option::default(),
                width: Option::default(),
                height: Option::default(),
                thumbnail: Option::default(),
                caption: Option::default(),
                parse_mode: Option::default(),
                caption_entities: Vec::default(),
                has_spoiler: bool::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
                reply_parameters: Option::default(),
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

    ///Animation to send. Pass a file\_id as String to send an animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an animation from the Internet, or upload a new animation using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub fn animation(mut self, animation: impl Into<InputFile>) -> Self {
        self.params.animation = animation.into();
        self
    }

    ///Duration of sent animation in seconds
    pub fn duration(mut self, duration: impl Into<i64>) -> Self {
        self.params.duration = Some(duration.into());
        self
    }

    ///Animation width
    pub fn width(mut self, width: impl Into<i64>) -> Self {
        self.params.width = Some(width.into());
        self
    }

    ///Animation height
    pub fn height(mut self, height: impl Into<i64>) -> Self {
        self.params.height = Some(height.into());
        self
    }

    ///Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    pub fn thumbnail(mut self, thumbnail: impl Into<InputFile>) -> Self {
        self.params.thumbnail = Some(thumbnail.into());
        self
    }

    ///Animation caption (may also be used when resending animation by *file\_id*), 0-1024 characters after entities parsing
    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.params.caption = Some(caption.into());
        self
    }

    ///Mode for parsing entities in the animation caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
        self.params.parse_mode = Some(parse_mode.into());
        self
    }

    ///A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
        self.params.caption_entities = caption_entities.into();
        self
    }

    ///Pass *True* if the animation needs to be covered with a spoiler animation
    pub fn has_spoiler(mut self, has_spoiler: impl Into<bool>) -> Self {
        self.params.has_spoiler = has_spoiler.into();
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

    ///Description of the message to reply to
    pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
        self.params.reply_parameters = Some(reply_parameters.into());
        self
    }

    ///Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl<'a> API {
    ///Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
    pub fn send_animation(
        &'a self,
        chat_id: impl Into<ChatId>,
        animation: impl Into<InputFile>,
    ) -> SendAnimationRequest {
        SendAnimationRequest::new(self, chat_id.into(), animation.into())
    }
}

// Divider: all content below this line will be preserved after code regen
