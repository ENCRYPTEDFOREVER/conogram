use serde::{Deserialize, Serialize};

use crate::{
    entities::{message_entity::MessageEntity, misc::chat_id::ChatId},
    utils::deserialize_utils::is_false,
};

/// Describes reply parameters for the message that is being sent.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#replyparameters)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyParameters {
    /// *Optional*. Identifier of the message that will be replied to in the current chat, or in the chat *chat\_id* if it is specified. Required if *ephemeral\_message\_id* isn't specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    /// *Optional*. If the message to be replied to is from a different chat, unique identifier for the chat or username of the bot, supergroup or channel in the format `@username`. Not supported for messages sent on behalf of a business account, messages from channel direct messages chats and ephemeral messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    /// *Optional*. Identifier of the incoming ephemeral message that will be replied to in the current chat. A reply to an ephemeral message must itself be an ephemeral message. An ephemeral message may only be replied to within 15 seconds of being sent. Required if *message\_id* isn't specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_message_id: Option<i64>,

    /// *Optional*. Pass *True* if the message should be sent even if the specified message to be replied to is not found. Always *False* for replies in another chat or forum topic, and sent ephemeral messages. Always *True* for messages sent on behalf of a business account.
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_sending_without_reply: bool,

    /// *Optional*. Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, *custom\_emoji*, and *date\_time* entities. The message will fail to send if the quote isn't found in the original message. Ignored for ephemeral messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,

    /// *Optional*. Mode for parsing entities in the quote. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<String>,

    /// *Optional*. A JSON-serialized list of special entities that appear in the quote. It can be specified instead of *quote\_parse\_mode*.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub quote_entities: Vec<MessageEntity>,

    /// *Optional*. Position of the quote in the original message in UTF-16 code units
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<i64>,

    /// *Optional*. Identifier of the specific checklist task to be replied to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_task_id: Option<i64>,

    /// *Optional*. Persistent identifier of the specific poll option to be replied to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll_option_id: Option<String>,
}

// Divider: all content below this line will be preserved after code regen

use std::range::Range;

use crate::entities::message::Message;

impl ReplyParameters {
    #[must_use]
    pub fn reply(message: &Message) -> Self {
        let message_id = if message.ephemeral_message_id.is_none() {
            Some(message.message_id)
        } else {
            None
        };

        Self {
            message_id,
            chat_id: None,
            ephemeral_message_id: message.ephemeral_message_id,
            allow_sending_without_reply: true,
            ..Default::default()
        }
    }

    /// If the message to be replied to is from a different chat, unique identifier for the chat or username of the bot, supergroup or channel in the format @username. Not supported for messages sent on behalf of a business account, messages from channel direct messages chats and ephemeral messages.
    pub fn set_original_chat(&mut self, chat_id: impl Into<ChatId>) {
        self.chat_id = Some(chat_id.into());
    }

    pub fn add_quote(&mut self, message: &Message, range: Option<impl Into<Range<usize>>>) {
        let (quote, quote_entities, quote_position) = if let Some(range) = range {
            let range = range.into();
            let range_start = range.start as i64;

            let (quote_text, quote_entities) = message
                .get_formatted_text()
                .unwrap_or_default()
                .slice(range)
                .build();

            (Some(quote_text), quote_entities, Some(range_start))
        } else {
            (None, vec![], None)
        };

        self.quote = quote;
        self.quote_entities = quote_entities;
        self.quote_position = quote_position;
    }

    pub const fn set_allow_sending_without_reply(&mut self, allow_sending_without_reply: bool) {
        self.allow_sending_without_reply = allow_sending_without_reply;
    }
}
