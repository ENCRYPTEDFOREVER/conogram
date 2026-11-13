use serde::{Deserialize, Serialize};

use crate::{
    entities::{message_entity::MessageEntity, rich_text::RichText},
    utils::deserialize_utils::is_false,
};

/// This object contains information about the quoted part of a message that is replied to by the given message.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#textquote)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TextQuote {
    /// Text of the quoted part of a message that is replied to by the given message; may be empty if the quote was chosen from a rich message
    pub text: String,

    /// *Optional*. Special entities that appear in the quote. Currently, only *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, *custom\_emoji*, and *date\_time* entities are kept in quotes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<MessageEntity>,

    /// *Optional*. Rich text of the quote if it was chosen from a [rich message](https://core.telegram.org/bots/api/#richmessage). May contain only plain text, [RichTextBold](https://core.telegram.org/bots/api/#richtextbold), [RichTextItalic](https://core.telegram.org/bots/api/#richtextitalic), [RichTextUnderline](https://core.telegram.org/bots/api/#richtextunderline), [RichTextStrikethrough](https://core.telegram.org/bots/api/#richtextstrikethrough), [RichTextSpoiler](https://core.telegram.org/bots/api/#richtextspoiler), [RichTextSubscript](https://core.telegram.org/bots/api/#richtextsubscript), [RichTextSuperscript](https://core.telegram.org/bots/api/#richtextsuperscript), [RichTextDateTime](https://core.telegram.org/bots/api/#richtextdatetime), [RichTextCustomEmoji](https://core.telegram.org/bots/api/#richtextcustomemoji), [RichTextMathematicalExpression](https://core.telegram.org/bots/api/#richtextmathematicalexpression), and arrays of them.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rich_texts: Vec<RichText>,

    /// Approximate quote position in the original message in UTF-16 code units as specified by the sender. For quotes from rich messages, it is the position in the text formed by joining plain texts of all quotable texts of the message, separated by line feeds.
    pub position: i64,

    /// *Optional*. *True*, if the quote was chosen manually by the message sender. Otherwise, the quote was added automatically by the server.
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_manual: bool,
}

// Divider: all content below this line will be preserved after code regen
