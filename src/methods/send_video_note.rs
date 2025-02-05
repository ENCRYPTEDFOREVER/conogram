use serde::Serialize;

use crate::{
    api::Api,
    entities::{
        message::Message,
        misc::{
            chat_id::ChatId,
            input_file::{GetFiles, InputFile},
            reply_markup::ReplyMarkup,
        },
        reply_parameters::ReplyParameters,
    },
    
    impl_into_future_multipart,
    request::RequestT,
    utils::deserialize_utils::is_false,
};

#[derive(Debug, Clone, Serialize)]

pub struct SendVideoNoteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub video_note: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,
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

impl GetFiles for SendVideoNoteParams {
    fn get_files(&self) -> Vec<&InputFile> {
        let mut vec = Vec::with_capacity(4);
        vec.push(&self.video_note);
        if let Some(thumbnail) = &self.thumbnail {
            vec.push(thumbnail);
        }
        vec
    }
}
impl_into_future_multipart!(SendVideoNoteRequest<'a>);

///As of [v.4.0](https://telegram.org/blog/video-messages-and-telescope), Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
#[derive(Clone)]
pub struct SendVideoNoteRequest<'a> {
    api: &'a Api,
    params: SendVideoNoteParams,
}

impl RequestT for SendVideoNoteRequest<'_> {
    type ParamsType = SendVideoNoteParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "sendVideoNote"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> SendVideoNoteRequest<'a> {
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>, video_note: impl Into<InputFile>) -> Self {
        Self {
            api,
            params: SendVideoNoteParams {
                chat_id: chat_id.into(),
                video_note: video_note.into(),
                business_connection_id: Option::default(),
                message_thread_id: Option::default(),
                duration: Option::default(),
                length: Option::default(),
                thumbnail: Option::default(),
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

    ///Video note to send. Pass a file\_id as String to send a video note that exists on the Telegram servers (recommended) or upload a new video using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Sending video notes by a URL is currently unsupported
    #[must_use]
    pub fn video_note(mut self, video_note: impl Into<InputFile>) -> Self {
        self.params.video_note = video_note.into();
        self
    }

    ///Duration of sent video in seconds
    #[must_use]
    pub fn duration(mut self, duration: impl Into<i64>) -> Self {
        self.params.duration = Some(duration.into());
        self
    }

    ///Video width and height, i.e. diameter of the video message
    #[must_use]
    pub fn length(mut self, length: impl Into<i64>) -> Self {
        self.params.length = Some(length.into());
        self
    }

    ///Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)
    #[must_use]
    pub fn thumbnail(mut self, thumbnail: impl Into<InputFile>) -> Self {
        self.params.thumbnail = Some(thumbnail.into());
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

impl Api {
    ///As of [v.4.0](https://telegram.org/blog/video-messages-and-telescope), Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn send_video_note(
        &self,
        chat_id: impl Into<ChatId>,
        video_note: impl Into<InputFile>,
    ) -> SendVideoNoteRequest {
        SendVideoNoteRequest::new(self, chat_id, video_note)
    }
}

// Divider: all content below this line will be preserved after code regen
