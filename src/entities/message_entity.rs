use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#messageentity)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageEntity {
    /// Type of the entity. Currently, can be “mention” (`@username`), “hashtag” (`#hashtag`), “cashtag” (`$USD`), “bot\_command” (`/start@jobs_bot`), “url” (`https://telegram.org`), “email” (`do-not-reply@telegram.org`), “phone\_number” (`+1-212-555-0123`), “bold” (**bold text**), “italic” (*italic text*), “underline” (underlined text), “strikethrough” (strikethrough text), “spoiler” (spoiler message), “blockquote” (block quotation), “expandable\_blockquote” (collapsed-by-default block quotation), “code” (monowidth string), “pre” (monowidth block), “text\_link” (for clickable text URLs), “text\_mention” (for users [without usernames](https://telegram.org/blog/edit#new-mentions)), “custom\_emoji” (for inline custom emoji stickers)
    #[serde(rename = "type")]
    pub type_: MessageEntityType,

    /// Offset in [UTF-16 code units](https://core.telegram.org/api/entities#entity-length) to the start of the entity
    pub offset: i64,

    /// Length of the entity in [UTF-16 code units](https://core.telegram.org/api/entities#entity-length)
    pub length: i64,

    /// *Optional*. For “text\_link” only, URL that will be opened after user taps on the text
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// *Optional*. For “text\_mention” only, the mentioned user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// *Optional*. For “pre” only, the programming language of the entity text
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// *Optional*. For “custom\_emoji” only, unique identifier of the custom emoji. Use [getCustomEmojiStickers](https://core.telegram.org/bots/api/#getcustomemojistickers) to get full information about the sticker
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}

/// Type of the entity. Currently, can be “mention” (`@username`), “hashtag” (`#hashtag`), “cashtag” (`$USD`), “bot\_command” (`/start@jobs_bot`), “url” (`https://telegram.org`), “email” (`do-not-reply@telegram.org`), “phone\_number” (`+1-212-555-0123`), “bold” (**bold text**), “italic” (*italic text*), “underline” (underlined text), “strikethrough” (strikethrough text), “spoiler” (spoiler message), “blockquote” (block quotation), “expandable\_blockquote” (collapsed-by-default block quotation), “code” (monowidth string), “pre” (monowidth block), “text\_link” (for clickable text URLs), “text\_mention” (for users [without usernames](https://telegram.org/blog/edit#new-mentions)), “custom\_emoji” (for inline custom emoji stickers)
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub enum MessageEntityType {
    /// `mention`
    #[default]
    #[serde(rename = "mention")]
    Mention,

    /// `hashtag`
    #[serde(rename = "hashtag")]
    Hashtag,

    /// `cashtag`
    #[serde(rename = "cashtag")]
    Cashtag,

    /// `bot_command`
    #[serde(rename = "bot_command")]
    BotCommand,

    /// `url`
    #[serde(rename = "url")]
    Url,

    /// `email`
    #[serde(rename = "email")]
    Email,

    /// `phone_number`
    #[serde(rename = "phone_number")]
    PhoneNumber,

    /// `bold`
    #[serde(rename = "bold")]
    Bold,

    /// `italic`
    #[serde(rename = "italic")]
    Italic,

    /// `underline`
    #[serde(rename = "underline")]
    Underline,

    /// `strikethrough`
    #[serde(rename = "strikethrough")]
    Strikethrough,

    /// `spoiler`
    #[serde(rename = "spoiler")]
    Spoiler,

    /// `blockquote`
    #[serde(rename = "blockquote")]
    Blockquote,

    /// `expandable_blockquote`
    #[serde(rename = "expandable_blockquote")]
    ExpandableBlockquote,

    /// `code`
    #[serde(rename = "code")]
    Code,

    /// `pre`
    #[serde(rename = "pre")]
    Pre,

    /// `text_link`
    #[serde(rename = "text_link")]
    TextLink,

    /// `text_mention`
    #[serde(rename = "text_mention")]
    TextMention,

    /// `custom_emoji`
    #[serde(rename = "custom_emoji")]
    CustomEmoji,
}

// Divider: all content below this line will be preserved after code regen

impl MessageEntity {
    pub fn get_text(&self, text: impl AsRef<str>) -> String {
        if self.offset < 0 || self.length < 0 {
            String::new()
        } else {
            let text = text.as_ref();
            let (start, end) = (
                text.utf16_offset_to_char_index(self.offset as usize),
                text.utf16_offset_to_char_index((self.offset + self.length) as usize),
            );

            text.chars().skip(start).take(end - start).collect()
        }
    }
}

pub trait Utf16Tools {
    /// Returns nearest char's index corresponding to provided offset in utf16 codeunits
    fn utf16_offset_to_char_index(&self, utf16_codeunit_offset: usize) -> usize;
}

impl<T: AsRef<str>> Utf16Tools for T {
    fn utf16_offset_to_char_index(&self, utf16_codeunit_offset: usize) -> usize {
        if utf16_codeunit_offset == 0 {
            return 0;
        }

        let text = self.as_ref();

        let mut utf8_char_index = 0;
        let mut utf16_len = 0;

        for char in text.chars() {
            utf16_len += char.len_utf16();
            utf8_char_index += 1;
            if utf16_len >= utf16_codeunit_offset {
                break;
            }
        }

        utf8_char_index
    }
}
