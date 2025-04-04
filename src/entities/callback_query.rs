use serde::{Deserialize, Serialize};

use crate::entities::{maybe_inaccessible_message::MaybeInaccessibleMessage, user::User};

/// This object represents an incoming callback query from a callback button in an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If the button that originated the query was attached to a message sent by the bot, the field *message* will be present. If the button was attached to a message sent via the bot (in [inline mode](https://core.telegram.org/bots/api/#inline-mode)), the field *inline\_message\_id* will be present. Exactly one of the fields *data* or *game\_short\_name* will be present.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#callbackquery)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,

    /// Sender
    pub from: User,

    /// *Optional*. Message sent by the bot with the callback button that originated the query
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<MaybeInaccessibleMessage>>,

    /// *Optional*. Identifier of the message sent via the bot in inline mode, that originated the query.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    /// Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in [games](https://core.telegram.org/bots/api/#games).
    pub chat_instance: String,

    /// *Optional*. Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// *Optional*. Short name of a [Game](https://core.telegram.org/bots/api/#games) to be returned, serves as the unique identifier for the game
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
use super::message::Message;
use crate::{api::Api, methods::answer_callback_query::AnswerCallbackQueryRequest};

impl CallbackQuery {
    pub fn answer<'a>(&'a self, api: &'a Api) -> AnswerCallbackQueryRequest<'a> {
        api.answer_callback_query(&self.id)
    }

    /// An alert will be shown by the client instead of a notification at the top of the chat screen
    pub fn alert<'a>(
        &'a self,
        api: &'a Api,
        text: impl Into<String>,
    ) -> AnswerCallbackQueryRequest<'a> {
        api.answer_callback_query(&self.id)
            .show_alert(true)
            .text(text)
    }

    /// Notification at the top of the chat screen will be shown instead of alert
    pub fn snackbar<'a>(
        &'a self,
        api: &'a Api,
        text: impl Into<String>,
    ) -> AnswerCallbackQueryRequest<'a> {
        api.answer_callback_query(&self.id)
            .show_alert(false)
            .text(text)
    }

    /// Backwards compatibility (Bot API <7.0)
    ///
    /// _Optional._ Message sent by the bot with the callback button that originated the query
    #[must_use]
    pub fn message(&self) -> Option<&Message> {
        self.message.as_ref().and_then(|m| m.as_ref().into())
    }
}
