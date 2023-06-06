use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::entities::keyboard_button::KeyboardButton;
use crate::entities::reply_keyboard_markup::ReplyKeyboardMarkup;
use crate::entities::reply_keyboard_remove::ReplyKeyboardRemove;
use crate::entities::{force_reply::ForceReply, inline_keyboard_button::InlineKeyboardButton};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ReplyMarkup {
    Keyboard(ReplyKeyboardMarkup),
    Inline(InlineKeyboardMarkup),
    ForceReply(ForceReply),
    Remove(ReplyKeyboardRemove),
}

impl ReplyMarkup {
    pub fn force_reply(
        input_field_placeholder: Option<impl Into<String>>,
        selective: bool,
    ) -> Self {
        Self::ForceReply(ForceReply {
            force_reply: true,
            input_field_placeholder: input_field_placeholder.map(|p| p.into()),
            selective,
        })
    }

    pub fn keyboard(
        keyboard: impl Into<Vec<Vec<KeyboardButton>>>,
        is_persistent: bool,
        resize_keyboard: bool,
        one_time_keyboard: bool,
        input_field_placeholder: Option<impl Into<String>>,
        selective: bool,
    ) -> Self {
        Self::Keyboard(ReplyKeyboardMarkup {
            keyboard: keyboard.into(),
            is_persistent,
            resize_keyboard,
            one_time_keyboard,
            input_field_placeholder: input_field_placeholder.map(|p| p.into()),
            selective,
        })
    }

    pub fn inline(keyboard: impl Into<Vec<Vec<InlineKeyboardButton>>>) -> Self {
        Self::Inline(InlineKeyboardMarkup {
            inline_keyboard: keyboard.into(),
        })
    }

    pub fn remove(remove_keyboard: bool, selective: bool) -> Self {
        Self::Remove(ReplyKeyboardRemove {
            remove_keyboard,
            selective,
        })
    }
}
