use crate::entities::message_entity::MessageEntity;
use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///This object contains information about the quoted part of a message that is replied to by the given message.
///API Reference: [link](https://core.telegram.org/bots/api/#textquote)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TextQuote {
    ///Text of the quoted part of a message that is replied to by the given message
    pub text: String,

    ///*Optional*. Special entities that appear in the quote. Currently, only *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, and *custom\_emoji* entities are kept in quotes.
    #[serde(default)]
    pub entities: Vec<MessageEntity>,

    ///Approximate quote position in the original message in UTF-16 code units as specified by the sender
    pub position: i64,

    ///*Optional*. True, if the quote was chosen manually by the message sender. Otherwise, the quote was added automatically by the server.
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_manual: bool,
}
// Divider: all content below this line will be preserved after code regen
