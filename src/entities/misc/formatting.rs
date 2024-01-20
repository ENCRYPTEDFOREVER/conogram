use std::{
    fmt::Display,
    ops::{Range, RangeBounds},
};

use crate::entities::{
    message_entity::{MessageEntity, MessageEntityType},
    user::User,
};

fn ranges_intersect<T1, T2>(one: Range<T1>, two: Range<T2>) -> bool
where
    T2: PartialOrd<T1>,
    T1: PartialOrd<T2>,
{
    two.contains(&one.start) || two.contains(&one.end)
}

pub trait Utf16Len {
    fn utf16_codeunits(&self) -> i64;
}

impl<T: AsRef<str>> Utf16Len for T {
    fn utf16_codeunits(&self) -> i64 {
        let mut len = 0;

        for byte in self.as_ref().bytes() {
            if (byte & 0xc0) != 0x80 {
                if byte >= 0xf0 {
                    len += 2;
                } else {
                    len += 1;
                }
            }
        }

        len
    }
}

#[derive(Debug, Clone)]
pub struct FormattedText {
    text: String,
    len: i64,
    entities: Vec<MessageEntity>,

    trim_spaces: bool,
    use_last_offsets: bool,

    last_offset: i64,
    last_len: i64,
}

impl PartialEq for FormattedText {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.len == other.len && self.entities == other.entities
    }
}

impl FormattedText {
    pub fn empty() -> Self {
        Self {
            text: String::new(),
            len: 0,
            entities: Vec::new(),
            trim_spaces: true,
            use_last_offsets: false,
            last_offset: 0,
            last_len: 0,
        }
    }

    pub fn new(text: impl Into<String>, entities: impl Into<Vec<MessageEntity>>) -> Self {
        let text = text.into();
        let len = text.utf16_codeunits();

        Self {
            text,
            len,
            entities: entities.into(),
            trim_spaces: true,
            use_last_offsets: false,
            last_offset: 0,
            last_len: len,
        }
    }

    pub fn slice(&self, range: impl Into<Range<usize>>) -> Self {
        let mut range: Range<usize> = range.into();
        range.end = std::cmp::min(range.end, self.len as usize);
        range.start = std::cmp::max(range.start, 0);

        let new_text = self.text[range.clone()].to_string();

        let mut new_entities = self
            .entities
            .iter()
            .filter(|ent| {
                ranges_intersect(
                    ent.offset..ent.offset + ent.length,
                    range.start as i64..range.end as i64,
                )
                // TODO: убедиться что похуй на бот команды и всякие хештеги
                // && ent.type_ != MessageEntityType::BotCommand
            })
            .cloned()
            .collect::<Vec<_>>();

        for ent in new_entities.iter_mut() {
            let offset = std::cmp::max(ent.offset, range.start as i64);
            let end = std::cmp::min(ent.offset + ent.length, range.end as i64);

            ent.offset = offset - range.start as i64;
            ent.length = end - offset;
        }

        Self::new(new_text, new_entities)
    }

    pub fn trim_spaces(mut self, trim: bool) -> Self {
        self.trim_spaces = trim;
        self
    }

    /// Uses offsets from last pushed text for all new ones and ignores new text content while the flag is set.
    pub fn ulo(mut self, use_last_offsets: bool) -> Self {
        self.use_last_offsets = use_last_offsets;
        self
    }

    fn calc_entity_len_offset(&self, text: &str) -> (i64, i64, i64) {
        let text_len = text.utf16_codeunits();

        let entity_offset;
        let entity_len;
        if self.trim_spaces {
            let l_trim = text.trim_start();
            let l_trim_len = l_trim.utf16_codeunits();

            entity_offset = self.len + (text_len - l_trim_len);
            entity_len = l_trim.trim_end().utf16_codeunits();
        } else {
            entity_offset = self.len;
            entity_len = text_len;
        }

        (text_len, entity_offset, entity_len)
    }

    fn push_entity_extended(
        mut self,
        text: impl AsRef<str>,
        entity_type: MessageEntityType,
        url: impl Into<Option<String>>,
        user: impl Into<Option<User>>,
        pre_lang: impl Into<Option<String>>,
        custom_emoji_id: impl Into<Option<String>>,
    ) -> Self {
        if self.use_last_offsets {
            let entity = MessageEntity {
                type_: entity_type,
                offset: self.last_offset,
                length: self.last_len,
                url: url.into(),
                user: user.into(),
                language: pre_lang.into(),
                custom_emoji_id: custom_emoji_id.into(),
            };

            self.entities.push(entity);
            self
        } else {
            let text = text.as_ref();
            let (text_len, entity_offset, entity_len) = self.calc_entity_len_offset(text);

            let entity = MessageEntity {
                type_: entity_type,
                offset: entity_offset,
                length: entity_len,
                url: url.into(),
                user: user.into(),
                language: pre_lang.into(),
                custom_emoji_id: custom_emoji_id.into(),
            };

            self.last_offset = entity_offset;
            self.last_len = entity_len;

            self.entities.push(entity);
            self.text.push_str(text);
            self.len += text_len;

            self
        }
    }

    fn push_entity_simple(mut self, text: impl AsRef<str>, entity_type: MessageEntityType) -> Self {
        if self.use_last_offsets {
            let entity = MessageEntity {
                type_: entity_type,
                offset: self.last_offset,
                length: self.last_len,
                ..Default::default()
            };

            self.entities.push(entity);
            self
        } else {
            let text = text.as_ref();
            let (text_len, entity_offset, entity_len) = self.calc_entity_len_offset(text);

            let entity = MessageEntity {
                type_: entity_type,
                offset: entity_offset,
                length: entity_len,
                ..Default::default()
            };

            self.last_offset = entity_offset;
            self.last_len = entity_len;

            self.entities.push(entity);
            self.text.push_str(text);
            self.len += text_len;
            self
        }
    }

    pub fn entities(
        mut self,
        text: impl AsRef<str>,
        entity_types: impl IntoIterator<Item = MessageEntityType>,
    ) -> Self {
        if self.use_last_offsets {
            let entities = entity_types.into_iter().map(|t| MessageEntity {
                type_: t,
                offset: self.last_offset,
                length: self.last_len,
                ..Default::default()
            });
            self.entities.extend(entities);
            self
        } else {
            let text = text.as_ref();
            let (text_len, entity_offset, entity_len) = self.calc_entity_len_offset(text);

            let entities = entity_types.into_iter().map(|t| MessageEntity {
                type_: t,
                offset: entity_offset,
                length: entity_len,
                ..Default::default()
            });

            self.last_offset = entity_offset;
            self.last_len = entity_len;

            self.entities.extend(entities);
            self.text.push_str(text);
            self.len += text_len;
            self
        }
    }

    pub fn text(mut self, text: impl AsRef<str>) -> Self {
        let text_ref = text.as_ref();
        let (text_len, entity_offset, entity_len) = self.calc_entity_len_offset(text_ref);

        if !self.use_last_offsets {
            self.last_offset = entity_offset;
            self.last_len = entity_len;
        }

        self.text.push_str(text_ref);
        self.len += text_len;
        self
    }

    pub fn nl(mut self) -> Self {
        // newlines don't count in entity borders, os no special handling for
        // self.use_last_entities_offset

        self.text.push('\n');
        self.len += 1;
        self
    }

    pub fn bold(self, text: impl ToString) -> Self {
        self.push_entity_simple(text.to_string(), MessageEntityType::Bold)
    }

    pub fn italic(self, text: impl ToString) -> Self {
        self.push_entity_simple(text.to_string(), MessageEntityType::Italic)
    }

    pub fn strikethrough(self, text: impl ToString) -> Self {
        self.push_entity_simple(text.to_string(), MessageEntityType::Strikethrough)
    }

    pub fn underline(self, text: impl ToString) -> Self {
        self.push_entity_simple(text.to_string(), MessageEntityType::Underline)
    }

    pub fn spoiler(self, text: impl ToString) -> Self {
        self.push_entity_simple(text.to_string(), MessageEntityType::Spoiler)
    }

    pub fn monowidth(self, text: impl ToString) -> Self {
        self.code(text)
    }

    pub fn code(self, text: impl ToString) -> Self {
        self.push_entity_simple(text.to_string(), MessageEntityType::Code)
    }

    pub fn pre(self, text: impl ToString) -> Self {
        self.code_block(text)
    }

    pub fn url(self, text: impl ToString, url: impl Into<String>) -> Self {
        self.push_entity_extended(
            text.to_string(),
            MessageEntityType::TextLink,
            Some(url.into()),
            None,
            None,
            None,
        )
    }

    pub fn mention(self, text: impl ToString, user_id: impl Into<i64>) -> Self {
        let user_id = user_id.into();
        if user_id > 0 {
            self.push_entity_extended(
                text.to_string(),
                MessageEntityType::TextMention,
                None,
                Some(User {
                    id: user_id.into(),
                    ..Default::default()
                }),
                None,
                None,
            )
        } else {
            self
        }
    }

    pub fn code_block(self, text: impl ToString) -> Self {
        self.push_entity_simple(text.to_string(), MessageEntityType::Pre)
    }

    pub fn code_block_in(self, text: impl ToString, language: impl Into<String>) -> Self {
        self.push_entity_extended(
            text.to_string(),
            MessageEntityType::Pre,
            None,
            None,
            Some(language.into().to_lowercase()),
            None,
        )
    }

    pub fn custom_emoji(self, text: impl ToString, custom_emoji_id: impl Into<String>) -> Self {
        self.push_entity_extended(
            text.to_string(),
            MessageEntityType::CustomEmoji,
            None,
            None,
            None,
            Some(custom_emoji_id.into()),
        )
    }

    pub fn build(self) -> (String, Vec<MessageEntity>) {
        (self.text, self.entities)
    }
}

impl Default for FormattedText {
    fn default() -> Self {
        Self::empty()
    }
}

impl Display for FormattedText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}
