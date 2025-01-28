use serde::Serialize;

use crate::{
    entities::{
        inline_keyboard_markup::InlineKeyboardMarkup, input_message_content::InputMessageContent,
        message_entity::MessageEntity,
    },
    utils::deserialize_utils::is_false,
};

/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the video.
///
/// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube), you **must** replace its content using *input\_message\_content*.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultvideo)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InlineQueryResultVideo {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,

    /// A valid URL for the embedded video player or video file
    pub video_url: String,

    /// MIME type of the content of the video URL, “text/html” or “video/mp4”
    pub mime_type: MimeType,

    /// URL of the thumbnail (JPEG only) for the video
    pub thumbnail_url: String,

    /// Title for the result
    pub title: String,

    /// *Optional*. Caption of the video to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// *Optional*. Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// *Optional*. Pass *True*, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "is_false")]
    pub show_caption_above_media: bool,

    /// *Optional*. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i64>,

    /// *Optional*. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i64>,

    /// *Optional*. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i64>,

    /// *Optional*. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// *Optional*. Content of the message to be sent instead of the video. This field is **required** if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// MIME type of the content of the video URL, “text/html” or “video/mp4”
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub enum MimeType {
    /// `text/html`
    #[default]
    #[serde(rename = "text/html")]
    TextHtml,

    /// `video/mp4`
    #[serde(rename = "video/mp4")]
    VideoMp4,
}

// Divider: all content below this line will be preserved after code regen
