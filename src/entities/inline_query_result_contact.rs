use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::entities::input_message_content::InputMessageContent;
use serde::Serialize;

///Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the contact.
///API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultcontact)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InlineQueryResultContact {
    ///Unique identifier for this result, 1-64 Bytes
    pub id: String,

    ///Contact's phone number
    pub phone_number: String,

    ///Contact's first name
    pub first_name: String,

    ///*Optional*. Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    ///*Optional*. Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,

    ///*Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    ///*Optional*. Content of the message to be sent instead of the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    ///*Optional*. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,

    ///*Optional*. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,

    ///*Optional*. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i64>,
}
// Divider: all content below this line will be preserved after code regen
