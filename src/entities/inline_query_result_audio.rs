use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::entities::input_message_content::InputMessageContent;
use crate::entities::message_entity::MessageEntity;
use serde::Serialize;

///Represents a link to an MP3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the audio.
///API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultaudio)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InlineQueryResultAudio {
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,

    ///A valid URL for the audio file
    pub audio_url: String,

    ///Title
    pub title: String,

    ///*Optional*. Caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    ///*Optional*. Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    ///*Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    ///*Optional*. Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,

    ///*Optional*. Audio duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<i64>,

    ///*Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    ///*Optional*. Content of the message to be sent instead of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
// Divider: all content below this line will be preserved after code regen
