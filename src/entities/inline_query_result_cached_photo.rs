use serde::Serialize;

use crate::{
    entities::{
        inline_keyboard_markup::InlineKeyboardMarkup, input_message_content::InputMessageContent,
        message_entity::MessageEntity,
    },
    utils::deserialize_utils::is_false,
};

/// Represents a link to a photo stored on the Telegram servers. By default, this photo will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the photo.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultcachedphoto)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InlineQueryResultCachedPhoto {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,

    /// A valid file identifier of the photo
    pub photo_file_id: String,

    /// *Optional*. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// *Optional*. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// *Optional*. Caption of the photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// *Optional*. Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
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

    /// *Optional*. Content of the message to be sent instead of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

// Divider: all content below this line will be preserved after code regen
