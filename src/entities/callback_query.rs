use crate::entities::message::Message;
use crate::entities::user::User;
use crate::utils::deserialize_utils::deserialize_boxed_option;
use serde::{Deserialize, Serialize};

///This object represents an incoming callback query from a callback button in an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If the button that originated the query was attached to a message sent by the bot, the field *message* will be present. If the button was attached to a message sent via the bot (in [inline mode](https://core.telegram.org/bots/api/#inline-mode)), the field *inline\_message\_id* will be present. Exactly one of the fields *data* or *game\_short\_name* will be present.
///API Reference: [link](https://core.telegram.org/bots/api/#callbackquery)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CallbackQuery {
    ///Unique identifier for this query
    pub id: String,

    ///Sender
    pub from: User,

    ///*Optional*. Message with the callback button that originated the query. Note that message content and message date will not be available if the message is too old
    #[serde(
        deserialize_with = "deserialize_boxed_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub message: Option<Box<Message>>,

    ///*Optional*. Identifier of the message sent via the bot in inline mode, that originated the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    ///Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in [games](https://core.telegram.org/bots/api/#games).
    pub chat_instance: String,

    ///*Optional*. Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    ///*Optional*. Short name of a [Game](https://core.telegram.org/bots/api/#games) to be returned, serves as the unique identifier for the game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}
// Divider: all content below this line will be preserved after code regen
use crate::api::API;
use crate::methods::answer_callback_query::AnswerCallbackQueryRequest;

impl CallbackQuery {
    pub fn answer<'a>(&'a self, api: &'a API) -> AnswerCallbackQueryRequest {
        api.answer_callback_query(&self.id)
    }

    /// An alert will be shown by the client instead of a notification at the top of the chat screen
    pub fn alert<'a>(
        &'a self,
        api: &'a API,
        text: impl Into<String>,
    ) -> AnswerCallbackQueryRequest {
        api.answer_callback_query(&self.id)
            .show_alert(true)
            .text(text)
    }

    /// Notification at the top of the chat screen will be shown instead of alert
    pub fn snackbar<'a>(
        &'a self,
        api: &'a API,
        text: impl Into<String>,
    ) -> AnswerCallbackQueryRequest {
        api.answer_callback_query(&self.id)
            .show_alert(false)
            .text(text)
    }
}
