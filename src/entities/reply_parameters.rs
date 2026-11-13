use serde::{Deserialize, Serialize};

use crate::{
    entities::{message_entity::MessageEntity, misc::chat_id::ChatId, rich_text::RichText},
    utils::deserialize_utils::is_false,
};

/// Describes reply parameters for the message that is being sent.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#replyparameters)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReplyParameters {
    /// *Optional*. Identifier of the message that will be replied to in the current chat, or in the chat *chat\_id* if it is specified. Required if *ephemeral\_message\_id* isn't specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    /// *Optional*. If the message to be replied to is from a different chat, unique identifier for the chat or username of the bot, supergroup, or channel in the format `@username`. Not supported for messages sent on behalf of a business account, messages from channel direct messages chats and ephemeral messages.
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

    /// *Optional*. A JSON-serialized list of quoted parts of the [rich message](https://core.telegram.org/bots/api/#richmessage) to be replied to. Must contain one element for each quoted text of the message in the order of their appearance. Each of the following is a separate quoted text: the text of a paragraph, a heading, a preformatted block, a footer, or a mathematical expression; the text and the credit of a block quotation or a pull quotation; the summary of a details block; the caption and each cell of a table; the text and the credit of a caption of a media block, a collage, a slideshow, or a map. Texts of blocks nested in lists, block quotations, details, collages, and slideshows are separate quoted texts too. The first and the last elements can contain only a part of the corresponding text; all other elements must contain the whole text. Formatting of the quoted parts must be kept, and rich texts must be nested in the same way as in the message received from the server. Mathematical expressions must be quoted entirely. Buttons in [RichBlockButtons](https://core.telegram.org/bots/api/#richblockbuttons), anchors and dividers can't be quoted. Up to 32 quoted texts with total length up to 1024 characters can be specified. Only plain text, [RichTextBold](https://core.telegram.org/bots/api/#richtextbold), [RichTextItalic](https://core.telegram.org/bots/api/#richtextitalic), [RichTextUnderline](https://core.telegram.org/bots/api/#richtextunderline), [RichTextStrikethrough](https://core.telegram.org/bots/api/#richtextstrikethrough), [RichTextSpoiler](https://core.telegram.org/bots/api/#richtextspoiler), [RichTextSubscript](https://core.telegram.org/bots/api/#richtextsubscript), [RichTextSuperscript](https://core.telegram.org/bots/api/#richtextsuperscript), [RichTextDateTime](https://core.telegram.org/bots/api/#richtextdatetime), [RichTextCustomEmoji](https://core.telegram.org/bots/api/#richtextcustomemoji), [RichTextMathematicalExpression](https://core.telegram.org/bots/api/#richtextmathematicalexpression), and arrays of them are kept in the quote; other rich text types are replaced with their text. If non-empty, then *quote*, *quote\_parse\_mode* and *quote\_entities* are ignored.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub quote_rich_texts: Vec<RichText>,

    /// *Optional*. Position of the quote in the original message in UTF-16 code units. For quotes from rich messages, it is the position in the text formed by joining plain texts of all quotable texts of the message, separated by line feeds.
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
