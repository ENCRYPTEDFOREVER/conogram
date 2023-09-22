use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SendChatActionParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub action: SendChatActionAction,
}

impl_into_future!(SendChatActionRequest<'a>);

///Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns *True* on success.
///
///Example: The [ImageBot](https://t.me/imagebot) needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use [sendChatAction](https://core.telegram.org/bots/api/#sendchataction) with *action* = *upload\_photo*. The user will see a “sending photo” status for the bot.
///
///We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.
#[derive(Clone)]
pub struct SendChatActionRequest<'a> {
    api: &'a API,
    params: SendChatActionParams,
}

impl<'a> RequestT for SendChatActionRequest<'a> {
    type ParamsType = SendChatActionParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "sendChatAction"
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
impl<'a> SendChatActionRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId, action: SendChatActionAction) -> Self {
        Self {
            api,
            params: SendChatActionParams {
                chat_id,
                action,
                message_thread_id: Option::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier for the target message thread; supergroups only
    pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
        self.params.message_thread_id = Some(message_thread_id.into());
        self
    }

    ///Type of action to broadcast. Choose one, depending on what the user is about to receive: *typing* for [text messages](https://core.telegram.org/bots/api/#sendmessage), *upload\_photo* for [photos](https://core.telegram.org/bots/api/#sendphoto), *record\_video* or *upload\_video* for [videos](https://core.telegram.org/bots/api/#sendvideo), *record\_voice* or *upload\_voice* for [voice notes](https://core.telegram.org/bots/api/#sendvoice), *upload\_document* for [general files](https://core.telegram.org/bots/api/#senddocument), *choose\_sticker* for [stickers](https://core.telegram.org/bots/api/#sendsticker), *find\_location* for [location data](https://core.telegram.org/bots/api/#sendlocation), *record\_video\_note* or *upload\_video\_note* for [video notes](https://core.telegram.org/bots/api/#sendvideonote).
    pub fn action(mut self, action: impl Into<SendChatActionAction>) -> Self {
        self.params.action = action.into();
        self
    }
}

impl<'a> API {
    ///Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns *True* on success.
    ///
    ///Example: The [ImageBot](https://t.me/imagebot) needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use [sendChatAction](https://core.telegram.org/bots/api/#sendchataction) with *action* = *upload\_photo*. The user will see a “sending photo” status for the bot.
    ///
    ///We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.
    pub fn send_chat_action(
        &'a self,
        chat_id: impl Into<ChatId>,
        action: impl Into<SendChatActionAction>,
    ) -> SendChatActionRequest {
        SendChatActionRequest::new(self, chat_id.into(), action.into())
    }
}

///Type of action to broadcast. Choose one, depending on what the user is about to receive: *typing* for [text messages](https://core.telegram.org/bots/api/#sendmessage), *upload\_photo* for [photos](https://core.telegram.org/bots/api/#sendphoto), *record\_video* or *upload\_video* for [videos](https://core.telegram.org/bots/api/#sendvideo), *record\_voice* or *upload\_voice* for [voice notes](https://core.telegram.org/bots/api/#sendvoice), *upload\_document* for [general files](https://core.telegram.org/bots/api/#senddocument), *choose\_sticker* for [stickers](https://core.telegram.org/bots/api/#sendsticker), *find\_location* for [location data](https://core.telegram.org/bots/api/#sendlocation), *record\_video\_note* or *upload\_video\_note* for [video notes](https://core.telegram.org/bots/api/#sendvideonote).
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "action")]
pub enum SendChatActionAction {
    #[default]
    /// "typing"
    #[serde(rename = "typing")]
    Typing,

    /// "upload_photo"
    #[serde(rename = "upload_photo")]
    UploadPhoto,

    /// "record_video"
    #[serde(rename = "record_video")]
    RecordVideo,

    /// "upload_video"
    #[serde(rename = "upload_video")]
    UploadVideo,

    /// "record_voice"
    #[serde(rename = "record_voice")]
    RecordVoice,

    /// "upload_voice"
    #[serde(rename = "upload_voice")]
    UploadVoice,

    /// "upload_document"
    #[serde(rename = "upload_document")]
    UploadDocument,

    /// "choose_sticker"
    #[serde(rename = "choose_sticker")]
    ChooseSticker,

    /// "find_location"
    #[serde(rename = "find_location")]
    FindLocation,

    /// "record_video_note"
    #[serde(rename = "record_video_note")]
    RecordVideoNote,

    /// "upload_video_note"
    #[serde(rename = "upload_video_note")]
    UploadVideoNote,
}

// Divider: all content below this line will be preserved after code regen
