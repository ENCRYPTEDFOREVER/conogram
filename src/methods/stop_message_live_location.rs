use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{
    inline_keyboard_markup::InlineKeyboardMarkup, message::Message, misc::chat_id::ChatId,
};

/// Use this method to stop updating a live location message before *live\_period* expires. On success, if the message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#stopmessagelivelocation)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Option<Message>)]
pub struct StopMessageLiveLocationParams {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    /// Required if *inline\_message\_id* is not specified. Identifier of the message with live location to stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    /// Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

// Divider: all content below this line will be preserved after code regen
