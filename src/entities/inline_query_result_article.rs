use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::entities::input_message_content::InputMessageContent;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;

///Represents a link to an article or web page.
///API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultarticle)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InlineQueryResultArticle {
    ///Unique identifier for this result, 1-64 Bytes
    pub id: String,

    ///Title of the result
    pub title: String,

    ///Content of the message to be sent
    pub input_message_content: InputMessageContent,

    ///*Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    ///*Optional*. URL of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    ///*Optional*. Pass *True* if you don't want the URL to be shown in the message
    #[serde(default, skip_serializing_if = "is_false")]
    pub hide_url: bool,

    ///*Optional*. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

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
use super::input_text_message_content::InputTextMessageContent;

impl InlineQueryResultArticle {
    /// Makes an article with provided title and text
    pub fn new_simple_text(
        id: impl Into<String>,
        title: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            input_message_content: InputTextMessageContent {
                message_text: text.into(),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        }
    }
}
