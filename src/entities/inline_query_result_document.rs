use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::entities::input_message_content::InputMessageContent;
use crate::entities::message_entity::MessageEntity;
use serde::Serialize;

///Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the file. Currently, only **.PDF** and **.ZIP** files can be sent using this method.
///API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultdocument)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InlineQueryResultDocument {
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,

    ///Title for the result
    pub title: String,

    ///*Optional*. Caption of the document to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    ///*Optional*. Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    ///*Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    ///A valid URL for the file
    pub document_url: String,

    ///MIME type of the content of the file, either “application/pdf” or “application/zip”
    pub mime_type: InlineQueryResultDocumentMimeType,

    ///*Optional*. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    ///*Optional*. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    ///*Optional*. Content of the message to be sent instead of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    ///*Optional*. URL of the thumbnail (JPEG only) for the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,

    ///*Optional*. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,

    ///*Optional*. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i64>,
}

///MIME type of the content of the file, either “application/pdf” or “application/zip”
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "mime_type")]
pub enum InlineQueryResultDocumentMimeType {
    #[default]
    /// "application/pdf"
    #[serde(rename = "application/pdf")]
    ApplicationPdf,

    /// "application/zip"
    #[serde(rename = "application/zip")]
    ApplicationZip,
}
// Divider: all content below this line will be preserved after code regen
