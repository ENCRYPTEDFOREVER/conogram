use crate::entities::message_entity::MessageEntity;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;

///Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of a text message to be sent as the result of an inline query.
///API Reference: [link](https://core.telegram.org/bots/api/#inputtextmessagecontent)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InputTextMessageContent {
    ///Text of the message to be sent, 1-4096 characters
    pub message_text: String,

    ///*Optional*. Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    ///*Optional*. List of special entities that appear in message text, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<MessageEntity>,

    ///*Optional*. Disables link previews for links in the sent message
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_web_page_preview: bool,
}
// Divider: all content below this line will be preserved after code regen
