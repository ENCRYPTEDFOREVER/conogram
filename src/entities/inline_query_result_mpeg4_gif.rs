use serde::Serialize;

use crate::{
    entities::{
        inline_keyboard_markup::InlineKeyboardMarkup, inline_query_result_gif::ThumbnailMimeType,
        input_message_content::InputMessageContent, message_entity::MessageEntity,
    },
    utils::deserialize_utils::is_false,
};

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the animation.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultmpeg4gif)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InlineQueryResultMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,

    /// A valid URL for the MPEG4 file
    pub mpeg4_url: String,

    /// *Optional*. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<i64>,

    /// *Optional*. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<i64>,

    /// *Optional*. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<i64>,

    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumbnail_url: String,

    /// *Optional*. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<ThumbnailMimeType>,

    /// *Optional*. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// *Optional*. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// *Optional*. Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// *Optional*. Pass *True*, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "is_false")]
    pub show_caption_above_media: bool,

    /// *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// *Optional*. Content of the message to be sent instead of the video animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

// Divider: all content below this line will be preserved after code regen
