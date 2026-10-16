use serde::{Deserialize, Serialize};

use crate::entities::{
    rich_message_button::RichMessageButton, rich_text_anchor::RichTextAnchor,
    rich_text_anchor_link::RichTextAnchorLink, rich_text_bank_card_number::RichTextBankCardNumber,
    rich_text_bold::RichTextBold, rich_text_bot_command::RichTextBotCommand,
    rich_text_button::RichTextButton, rich_text_cashtag::RichTextCashtag,
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
/// * [RichTextButton](https://core.telegram.org/bots/api/#richtextbutton)
/// * [RichTextAnchor](https://core.telegram.org/bots/api/#richtextanchor)
/// * [RichTextAnchorLink](https://core.telegram.org/bots/api/#richtextanchorlink)
/// * [RichTextReference](https://core.telegram.org/bots/api/#richtextreference)
/// * [RichTextReferenceLink](https://core.telegram.org/bots/api/#richtextreferencelink)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtext)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RichText {
    /// A bold text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextbold)
    Bold(RichTextBold),

    /// An italicized text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextitalic)
    Italic(RichTextItalic),

    /// An underlined text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextunderline)
    Underline(RichTextUnderline),

    /// A strikethrough text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextstrikethrough)
    Strikethrough(RichTextStrikethrough),

    /// A text covered by a spoiler.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextspoiler)
    Spoiler(RichTextSpoiler),

    /// Formatted date and time.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextdatetime)
    DateTime(RichTextDateTime),

    /// A mention of a Telegram user by their identifier.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtexttextmention)
    TextMention(RichTextTextMention),

    /// A subscript text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextsubscript)
    Subscript(RichTextSubscript),

    /// A superscript text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextsuperscript)
    Superscript(RichTextSuperscript),

    /// A marked text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextmarked)
    Marked(RichTextMarked),

    /// A monowidth text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextcode)
    Code(RichTextCode),

    /// A custom emoji.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextcustomemoji)
    CustomEmoji(RichTextCustomEmoji),

    /// A mathematical expression.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextmathematicalexpression)
    MathematicalExpression(RichTextMathematicalExpression),

    /// A text with a link.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtexturl)
    Url(RichTextUrl),

    /// A text with an email address.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextemailaddress)
    EmailAddress(RichTextEmailAddress),

    /// A text with a phone number.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextphonenumber)
    PhoneNumber(RichTextPhoneNumber),

    /// A text with a bank card number.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextbankcardnumber)
    BankCardNumber(RichTextBankCardNumber),

    /// A mention by a username.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextmention)
    Mention(RichTextMention),

    /// A hashtag.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtexthashtag)
    Hashtag(RichTextHashtag),

    /// A cashtag.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextcashtag)
    Cashtag(RichTextCashtag),

    /// A bot command.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextbotcommand)
    BotCommand(RichTextBotCommand),

    /// A button.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextbutton)
    Button(RichTextButton),

    /// An anchor.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextanchor)
    Anchor(RichTextAnchor),

    /// A link to an anchor.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextanchorlink)
    AnchorLink(RichTextAnchorLink),

    /// A reference.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextreference)
    Reference(RichTextReference),

    /// A link to a reference.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextreferencelink)
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
        Self::Plain(String::default())
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

impl From<RichTextButton> for RichText {
    fn from(value: RichTextButton) -> Self {
        Self::Button(value)
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

impl From<RichMessageButton> for RichText {
    fn from(value: RichMessageButton) -> Self {
        Self::Button(RichTextButton { button: value })
    }
}

#[macro_export]
macro_rules! rich_text {
    ($($b: expr),*$(,)?) => {
        $crate::entities::rich_text::RichText::Array(vec![
            $(
                Box::new($b.into())
            ),*
        ])
    };
}

impl RichText {
    /// A bold text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextbold)
    pub fn bold(text: impl Into<Self>) -> Self {
        Self::Bold(RichTextBold {
            text: Box::new(text.into()),
        })
    }

    /// An italicized text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextitalic)
    pub fn italic(text: impl Into<Self>) -> Self {
        Self::Italic(RichTextItalic {
            text: Box::new(text.into()),
        })
    }

    /// An underlined text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextunderline)
    pub fn underline(text: impl Into<Self>) -> Self {
        Self::Underline(RichTextUnderline {
            text: Box::new(text.into()),
        })
    }

    /// A strikethrough text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextstrikethrough)
    pub fn strikethrough(text: impl Into<Self>) -> Self {
        Self::Strikethrough(RichTextStrikethrough {
            text: Box::new(text.into()),
        })
    }

    /// A text covered by a spoiler.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextspoiler)
    pub fn spoiler(text: impl Into<Self>) -> Self {
        Self::Spoiler(RichTextSpoiler {
            text: Box::new(text.into()),
        })
    }

    /// Formatted date and time.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextdatetime)
    pub fn datetime(text: impl Into<Self>, unix_time: i64, format: impl Into<String>) -> Self {
        Self::DateTime(RichTextDateTime {
            text: Box::new(text.into()),
            unix_time,
            date_time_format: format.into(),
        })
    }

    /// A mention of a Telegram user by their identifier.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtexttextmention)
    pub fn mention(text: impl Into<Self>, user_id: impl Into<i64>) -> Self {
        Self::TextMention(RichTextTextMention {
            text: Box::new(text.into()),
            user: super::user::User {
                id: user_id.into(),
                ..Default::default()
            },
        })
    }

    /// A subscript text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextsubscript)
    pub fn subscript(text: impl Into<Self>) -> Self {
        Self::Subscript(RichTextSubscript {
            text: Box::new(text.into()),
        })
    }

    /// A superscript text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextsuperscript)
    pub fn superscript(text: impl Into<Self>) -> Self {
        Self::Superscript(RichTextSuperscript {
            text: Box::new(text.into()),
        })
    }

    /// A marked text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextmarked)
    pub fn marked(text: impl Into<Self>) -> Self {
        Self::Marked(RichTextMarked {
            text: Box::new(text.into()),
        })
    }

    /// A monowidth text.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextcode)
    pub fn code(text: impl Into<Self>) -> Self {
        Self::Code(RichTextCode {
            text: Box::new(text.into()),
        })
    }

    /// A custom emoji.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextcustomemoji)
    pub fn custom_emoji(custom_emoji_id: impl Into<String>, alt_text: impl Into<String>) -> Self {
        Self::CustomEmoji(RichTextCustomEmoji {
            custom_emoji_id: custom_emoji_id.into(),
            alternative_text: alt_text.into(),
        })
    }

    /// A mathematical expression.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextmathematicalexpression)
    pub fn math(expression: impl Into<String>) -> Self {
        Self::MathematicalExpression(RichTextMathematicalExpression {
            expression: expression.into(),
        })
    }

    /// A text with a link.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtexturl)
    pub fn url(text: impl Into<Self>, url: impl Into<String>) -> Self {
        Self::Url(RichTextUrl {
            text: Box::new(text.into()),
            url: url.into(),
        })
    }

    /// A text with an email address.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextemailaddress)
    pub fn email(text: impl Into<Self>, email: impl Into<String>) -> Self {
        Self::EmailAddress(RichTextEmailAddress {
            text: Box::new(text.into()),
            email_address: email.into(),
        })
    }

    /// A text with a phone number.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextphonenumber)
    pub fn phone_number(text: impl Into<Self>, phone_number: impl Into<String>) -> Self {
        Self::PhoneNumber(RichTextPhoneNumber {
            text: Box::new(text.into()),
            phone_number: phone_number.into(),
        })
    }

    /// A text with a bank card number.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextbankcardnumber)
    pub fn bank_card(text: impl Into<Self>, card_number: impl Into<String>) -> Self {
        Self::BankCardNumber(RichTextBankCardNumber {
            text: Box::new(text.into()),
            bank_card_number: card_number.into(),
        })
    }

    /// A mention by a username.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextmention)
    pub fn mention_username(text: impl Into<Self>, username: impl Into<String>) -> Self {
        Self::Mention(RichTextMention {
            text: Box::new(text.into()),
            username: username.into(),
        })
    }

    /// A hashtag.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtexthashtag)
    pub fn hashtag(text: impl Into<Self>, hashtag: impl Into<String>) -> Self {
        Self::Hashtag(RichTextHashtag {
            text: Box::new(text.into()),
            hashtag: hashtag.into(),
        })
    }

    /// A cashtag.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextcashtag)
    pub fn cashtag(text: impl Into<Self>, cashtag: impl Into<String>) -> Self {
        Self::Cashtag(RichTextCashtag {
            text: Box::new(text.into()),
            cashtag: cashtag.into(),
        })
    }

    /// A bot command.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextbotcommand)
    pub fn bot_command(text: impl Into<Self>, command: impl Into<String>) -> Self {
        Self::BotCommand(RichTextBotCommand {
            text: Box::new(text.into()),
            bot_command: command.into(),
        })
    }

    /// A button.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextbutton)
    pub fn button(button: impl Into<RichMessageButton>) -> Self {
        Self::Button(RichTextButton {
            button: button.into(),
        })
    }

    /// An anchor.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextanchor)
    pub fn anchor(name: impl Into<String>) -> Self {
        Self::Anchor(RichTextAnchor { name: name.into() })
    }

    /// A link to an anchor.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextanchorlink)
    pub fn anchor_link(text: impl Into<Self>, anchor_name: impl Into<String>) -> Self {
        Self::AnchorLink(RichTextAnchorLink {
            text: Box::new(text.into()),
            anchor_name: anchor_name.into(),
        })
    }

    /// A reference.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextreference)
    pub fn reference(text: impl Into<Self>, name: impl Into<String>) -> Self {
        Self::Reference(RichTextReference {
            text: Box::new(text.into()),
            name: name.into(),
        })
    }

    /// A link to a reference.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richtextreferencelink)
    pub fn reference_link(text: impl Into<Self>, ref_name: impl Into<String>) -> Self {
        Self::ReferenceLink(RichTextReferenceLink {
            text: Box::new(text.into()),
            reference_name: ref_name.into(),
        })
    }
}
