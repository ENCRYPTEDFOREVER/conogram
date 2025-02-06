use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns *True* on success.
///
/// Example: The [ImageBot](https://t.me/imagebot) needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use [sendChatAction](https://core.telegram.org/bots/api/#sendchataction) with *action* = *upload\_photo*. The user will see a “sending photo” status for the bot.
///
/// We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#sendchataction)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SendChatActionParams {
    /// Unique identifier of the business connection on behalf of which the action will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Unique identifier for the target message thread; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// Type of action to broadcast. Choose one, depending on what the user is about to receive: *typing* for [text messages](https://core.telegram.org/bots/api/#sendmessage), *upload\_photo* for [photos](https://core.telegram.org/bots/api/#sendphoto), *record\_video* or *upload\_video* for [videos](https://core.telegram.org/bots/api/#sendvideo), *record\_voice* or *upload\_voice* for [voice notes](https://core.telegram.org/bots/api/#sendvoice), *upload\_document* for [general files](https://core.telegram.org/bots/api/#senddocument), *choose\_sticker* for [stickers](https://core.telegram.org/bots/api/#sendsticker), *find\_location* for [location data](https://core.telegram.org/bots/api/#sendlocation), *record\_video\_note* or *upload\_video\_note* for [video notes](https://core.telegram.org/bots/api/#sendvideonote).
    pub action: ChatAction,
}

/// Type of action to broadcast. Choose one, depending on what the user is about to receive: *typing* for [text messages](https://core.telegram.org/bots/api/#sendmessage), *upload\_photo* for [photos](https://core.telegram.org/bots/api/#sendphoto), *record\_video* or *upload\_video* for [videos](https://core.telegram.org/bots/api/#sendvideo), *record\_voice* or *upload\_voice* for [voice notes](https://core.telegram.org/bots/api/#sendvoice), *upload\_document* for [general files](https://core.telegram.org/bots/api/#senddocument), *choose\_sticker* for [stickers](https://core.telegram.org/bots/api/#sendsticker), *find\_location* for [location data](https://core.telegram.org/bots/api/#sendlocation), *record\_video\_note* or *upload\_video\_note* for [video notes](https://core.telegram.org/bots/api/#sendvideonote).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub enum ChatAction {
    /// `typing`
    #[default]
    #[serde(rename = "typing")]
    Typing,

    /// `upload_photo`
    #[serde(rename = "upload_photo")]
    UploadPhoto,

    /// `record_video`
    #[serde(rename = "record_video")]
    RecordVideo,

    /// `upload_video`
    #[serde(rename = "upload_video")]
    UploadVideo,

    /// `record_voice`
    #[serde(rename = "record_voice")]
    RecordVoice,

    /// `upload_voice`
    #[serde(rename = "upload_voice")]
    UploadVoice,

    /// `upload_document`
    #[serde(rename = "upload_document")]
    UploadDocument,

    /// `choose_sticker`
    #[serde(rename = "choose_sticker")]
    ChooseSticker,

    /// `find_location`
    #[serde(rename = "find_location")]
    FindLocation,

    /// `record_video_note`
    #[serde(rename = "record_video_note")]
    RecordVideoNote,

    /// `upload_video_note`
    #[serde(rename = "upload_video_note")]
    UploadVideoNote,
}

// Divider: all content below this line will be preserved after code regen
