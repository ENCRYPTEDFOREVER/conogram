use crate::entities::message_entity::MessageEntity;
use crate::entities::misc::chat_id::ChatId;
use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///Describes reply parameters for the message that is being sent.
///
///API Reference: [link](https://core.telegram.org/bots/api/#replyparameters)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReplyParameters {
    ///Identifier of the message that will be replied to in the current chat, or in the chat *chat\_id* if it is specified
    pub message_id: i64,

    ///*Optional*. If the message to be replied to is from a different chat, unique identifier for the chat or username of the channel (in the format `@channelusername`). Not supported for messages sent on behalf of a business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    ///*Optional*. Pass *True* if the message should be sent even if the specified message to be replied to is not found. Always *False* for replies in another chat or forum topic. Always *True* for messages sent on behalf of a business account.
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_sending_without_reply: bool,

    ///*Optional*. Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, and *custom\_emoji* entities. The message will fail to send if the quote isn't found in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,

    ///*Optional*. Mode for parsing entities in the quote. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<String>,

    ///*Optional*. A JSON-serialized list of special entities that appear in the quote. It can be specified instead of *quote\_parse\_mode*.
    #[serde(default)]
    pub quote_entities: Vec<MessageEntity>,

    ///*Optional*. Position of the quote in the original message in UTF-16 code units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<i64>,
}
// Divider: all content below this line will be preserved after code regen
impl ReplyParameters {
    pub fn new_current_chat(message_id: impl Into<i64>) -> Self {
        Self {
            message_id: message_id.into(),
            allow_sending_without_reply: true,
            ..Default::default()
        }
    }

    pub fn new_other_chat(message_id: impl Into<i64>, chat_id: impl Into<ChatId>) -> Self {
        Self {
            message_id: message_id.into(),
            chat_id: Some(chat_id.into()),
            allow_sending_without_reply: true,
            ..Default::default()
        }
    }

    #[must_use]
    pub const fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = allow_sending_without_reply;
        self
    }

    #[must_use]
    pub fn chat(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.chat_id = Some(chat_id.into());
        self
    }
}
