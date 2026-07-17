use serde::{Deserialize, Serialize};

use crate::entities::{
    rich_text_anchor::RichTextAnchor, rich_text_anchor_link::RichTextAnchorLink,
    rich_text_bank_card_number::RichTextBankCardNumber, rich_text_bold::RichTextBold,
    rich_text_bot_command::RichTextBotCommand, rich_text_cashtag::RichTextCashtag,
    rich_text_code::RichTextCode, rich_text_custom_emoji::RichTextCustomEmoji,
    rich_text_date_time::RichTextDateTime, rich_text_email_address::RichTextEmailAddress,
    rich_text_hashtag::RichTextHashtag, rich_text_italic::RichTextItalic,
    rich_text_marked::RichTextMarked,
    rich_text_mathematical_expression::RichTextMathematicalExpression,
    rich_text_mention::RichTextMention, rich_text_phone_number::RichTextPhoneNumber,
    rich_text_reference::RichTextReference, rich_text_reference_link::RichTextReferenceLink,
    rich_text_spoiler::RichTextSpoiler, rich_text_strikethrough::RichTextStrikethrough,
    rich_text_subscript::RichTextSubscript, rich_text_superscript::RichTextSuperscript,
    rich_text_text_mention::RichTextTextMention, rich_text_underline::RichTextUnderline,
    rich_text_url::RichTextUrl,
};

/// This object represents a rich formatted text. Currently, it can be either a String for plain text, an Array of [RichText](https://core.telegram.org/bots/api/#richtext), or any of the following types:
///
/// * [RichTextBold](https://core.telegram.org/bots/api/#richtextbold)
/// * [RichTextItalic](https://core.telegram.org/bots/api/#richtextitalic)
/// * [RichTextUnderline](https://core.telegram.org/bots/api/#richtextunderline)
/// * [RichTextStrikethrough](https://core.telegram.org/bots/api/#richtextstrikethrough)
/// * [RichTextSpoiler](https://core.telegram.org/bots/api/#richtextspoiler)
/// * [RichTextDateTime](https://core.telegram.org/bots/api/#richtextdatetime)
/// * [RichTextTextMention](https://core.telegram.org/bots/api/#richtexttextmention)
/// * [RichTextSubscript](https://core.telegram.org/bots/api/#richtextsubscript)
/// * [RichTextSuperscript](https://core.telegram.org/bots/api/#richtextsuperscript)
/// * [RichTextMarked](https://core.telegram.org/bots/api/#richtextmarked)
/// * [RichTextCode](https://core.telegram.org/bots/api/#richtextcode)
/// * [RichTextCustomEmoji](https://core.telegram.org/bots/api/#richtextcustomemoji)
/// * [RichTextMathematicalExpression](https://core.telegram.org/bots/api/#richtextmathematicalexpression)
/// * [RichTextUrl](https://core.telegram.org/bots/api/#richtexturl)
/// * [RichTextEmailAddress](https://core.telegram.org/bots/api/#richtextemailaddress)
/// * [RichTextPhoneNumber](https://core.telegram.org/bots/api/#richtextphonenumber)
/// * [RichTextBankCardNumber](https://core.telegram.org/bots/api/#richtextbankcardnumber)
/// * [RichTextMention](https://core.telegram.org/bots/api/#richtextmention)
/// * [RichTextHashtag](https://core.telegram.org/bots/api/#richtexthashtag)
/// * [RichTextCashtag](https://core.telegram.org/bots/api/#richtextcashtag)
/// * [RichTextBotCommand](https://core.telegram.org/bots/api/#richtextbotcommand)
/// * [RichTextAnchor](https://core.telegram.org/bots/api/#richtextanchor)
/// * [RichTextAnchorLink](https://core.telegram.org/bots/api/#richtextanchorlink)
/// * [RichTextReference](https://core.telegram.org/bots/api/#richtextreference)
/// * [RichTextReferenceLink](https://core.telegram.org/bots/api/#richtextreferencelink)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtext)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RichText {
    /// A bold text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextbold)
    #[serde(rename = "bold")]
    Bold(RichTextBold),

    /// An italicized text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextitalic)
    #[serde(rename = "italic")]
    Italic(RichTextItalic),

    /// An underlined text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextunderline)
    #[serde(rename = "underline")]
    Underline(RichTextUnderline),

    /// A strikethrough text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextstrikethrough)
    #[serde(rename = "strikethrough")]
    Strikethrough(RichTextStrikethrough),

    /// A text covered by a spoiler.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextspoiler)
    #[serde(rename = "spoiler")]
    Spoiler(RichTextSpoiler),

    /// Formatted date and time.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextdatetime)
    #[serde(rename = "date_time")]
    DateTime(RichTextDateTime),

    /// A mention of a Telegram user by their identifier.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtexttextmention)
    #[serde(rename = "text_mention")]
    TextMention(RichTextTextMention),

    /// A subscript text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextsubscript)
    #[serde(rename = "subscript")]
    Subscript(RichTextSubscript),

    /// A superscript text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextsuperscript)
    #[serde(rename = "superscript")]
    Superscript(RichTextSuperscript),

    /// A marked text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextmarked)
    #[serde(rename = "marked")]
    Marked(RichTextMarked),

    /// A monowidth text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextcode)
    #[serde(rename = "code")]
    Code(RichTextCode),

    /// A custom emoji.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextcustomemoji)
    #[serde(rename = "custom_emoji")]
    CustomEmoji(RichTextCustomEmoji),

    /// A mathematical expression.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextmathematicalexpression)
    #[serde(rename = "mathematical_expression")]
    MathematicalExpression(RichTextMathematicalExpression),

    /// A text with a link.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtexturl)
    #[serde(rename = "url")]
    Url(RichTextUrl),

    /// A text with an email address.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextemailaddress)
    #[serde(rename = "email_address")]
    EmailAddress(RichTextEmailAddress),

    /// A text with a phone number.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextphonenumber)
    #[serde(rename = "phone_number")]
    PhoneNumber(RichTextPhoneNumber),

    /// A text with a bank card number.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextbankcardnumber)
    #[serde(rename = "bank_card_number")]
    BankCardNumber(RichTextBankCardNumber),

    /// A mention by a username.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextmention)
    #[serde(rename = "mention")]
    Mention(RichTextMention),

    /// A hashtag.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtexthashtag)
    #[serde(rename = "hashtag")]
    Hashtag(RichTextHashtag),

    /// A cashtag.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextcashtag)
    #[serde(rename = "cashtag")]
    Cashtag(RichTextCashtag),

    /// A bot command.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextbotcommand)
    #[serde(rename = "bot_command")]
    BotCommand(RichTextBotCommand),

    /// An anchor.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextanchor)
    #[serde(rename = "anchor")]
    Anchor(RichTextAnchor),

    /// A link to an anchor.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextanchorlink)
    #[serde(rename = "anchor_link")]
    AnchorLink(RichTextAnchorLink),

    /// A reference.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextreference)
    #[serde(rename = "reference")]
    Reference(RichTextReference),

    /// A link to a reference.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextreferencelink)
    #[serde(rename = "reference_link")]
    ReferenceLink(RichTextReferenceLink),

    /// Bruh
    #[serde(untagged)]
    Array(Vec<Box<Self>>),

    /// Plain text
    #[serde(untagged)]
    Plain(String),
}

impl Default for RichText {
    fn default() -> Self {
        Self::Bold(RichTextBold::default())
    }
}

impl From<RichTextBold> for RichText {
    fn from(value: RichTextBold) -> Self {
        Self::Bold(value)
    }
}

impl From<RichTextItalic> for RichText {
    fn from(value: RichTextItalic) -> Self {
        Self::Italic(value)
    }
}

impl From<RichTextUnderline> for RichText {
    fn from(value: RichTextUnderline) -> Self {
        Self::Underline(value)
    }
}

impl From<RichTextStrikethrough> for RichText {
    fn from(value: RichTextStrikethrough) -> Self {
        Self::Strikethrough(value)
    }
}

impl From<RichTextSpoiler> for RichText {
    fn from(value: RichTextSpoiler) -> Self {
        Self::Spoiler(value)
    }
}

impl From<RichTextDateTime> for RichText {
    fn from(value: RichTextDateTime) -> Self {
        Self::DateTime(value)
    }
}

impl From<RichTextTextMention> for RichText {
    fn from(value: RichTextTextMention) -> Self {
        Self::TextMention(value)
    }
}

impl From<RichTextSubscript> for RichText {
    fn from(value: RichTextSubscript) -> Self {
        Self::Subscript(value)
    }
}

impl From<RichTextSuperscript> for RichText {
    fn from(value: RichTextSuperscript) -> Self {
        Self::Superscript(value)
    }
}

impl From<RichTextMarked> for RichText {
    fn from(value: RichTextMarked) -> Self {
        Self::Marked(value)
    }
}

impl From<RichTextCode> for RichText {
    fn from(value: RichTextCode) -> Self {
        Self::Code(value)
    }
}

impl From<RichTextCustomEmoji> for RichText {
    fn from(value: RichTextCustomEmoji) -> Self {
        Self::CustomEmoji(value)
    }
}

impl From<RichTextMathematicalExpression> for RichText {
    fn from(value: RichTextMathematicalExpression) -> Self {
        Self::MathematicalExpression(value)
    }
}

impl From<RichTextUrl> for RichText {
    fn from(value: RichTextUrl) -> Self {
        Self::Url(value)
    }
}

impl From<RichTextEmailAddress> for RichText {
    fn from(value: RichTextEmailAddress) -> Self {
        Self::EmailAddress(value)
    }
}

impl From<RichTextPhoneNumber> for RichText {
    fn from(value: RichTextPhoneNumber) -> Self {
        Self::PhoneNumber(value)
    }
}

impl From<RichTextBankCardNumber> for RichText {
    fn from(value: RichTextBankCardNumber) -> Self {
        Self::BankCardNumber(value)
    }
}

impl From<RichTextMention> for RichText {
    fn from(value: RichTextMention) -> Self {
        Self::Mention(value)
    }
}

impl From<RichTextHashtag> for RichText {
    fn from(value: RichTextHashtag) -> Self {
        Self::Hashtag(value)
    }
}

impl From<RichTextCashtag> for RichText {
    fn from(value: RichTextCashtag) -> Self {
        Self::Cashtag(value)
    }
}

impl From<RichTextBotCommand> for RichText {
    fn from(value: RichTextBotCommand) -> Self {
        Self::BotCommand(value)
    }
}

impl From<RichTextAnchor> for RichText {
    fn from(value: RichTextAnchor) -> Self {
        Self::Anchor(value)
    }
}

impl From<RichTextAnchorLink> for RichText {
    fn from(value: RichTextAnchorLink) -> Self {
        Self::AnchorLink(value)
    }
}

impl From<RichTextReference> for RichText {
    fn from(value: RichTextReference) -> Self {
        Self::Reference(value)
    }
}

impl From<RichTextReferenceLink> for RichText {
    fn from(value: RichTextReferenceLink) -> Self {
        Self::ReferenceLink(value)
    }
}

// Divider: all content below this line will be preserved after code regen

impl<T: Into<String>> From<T> for RichText {
    fn from(value: T) -> Self {
        Self::Plain(value.into())
    }
}
