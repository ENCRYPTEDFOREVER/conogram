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

impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(value: ReplyKeyboardMarkup) -> Self {
        Self::Keyboard(value)
    }
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(value: InlineKeyboardMarkup) -> Self {
        Self::Inline(value)
    }
}

impl From<ForceReply> for ReplyMarkup {
    fn from(value: ForceReply) -> Self {
        Self::ForceReply(value)
    }
}

impl From<ReplyKeyboardRemove> for ReplyMarkup {
    fn from(value: ReplyKeyboardRemove) -> Self {
        Self::Remove(value)
    }
}

impl ReplyMarkup {
    ///Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice [privacy mode](https://core.telegram.org/bots/features#privacy-mode).
    ///API Reference: [link](https://core.telegram.org/bots/api/#forcereply)
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

    ///This object represents a [custom keyboard](https://core.telegram.org/bots/features#keyboards) with reply options (see [Introduction to bots](https://core.telegram.org/bots/features#keyboards) for details and examples).
    ///API Reference: [link](https://core.telegram.org/bots/api/#replykeyboardmarkup)
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

    ///This object represents an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) that appears right next to the message it belongs to.
    ///API Reference: [link](https://core.telegram.org/bots/api/#inlinekeyboardmarkup)
    pub fn inline(keyboard: impl Into<Vec<Vec<InlineKeyboardButton>>>) -> Self {
        Self::Inline(InlineKeyboardMarkup {
            inline_keyboard: keyboard.into(),
        })
    }

    ///Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see [ReplyKeyboardMarkup](https://core.telegram.org/bots/api/#replykeyboardmarkup)).
    ///API Reference: [link](https://core.telegram.org/bots/api/#replykeyboardremove)
    pub fn remove(remove_keyboard: bool, selective: bool) -> Self {
        Self::Remove(ReplyKeyboardRemove {
            remove_keyboard,
            selective,
        })
    }
}
