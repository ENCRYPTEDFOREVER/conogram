use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{
    inline_keyboard_markup::InlineKeyboardMarkup, message::Message, misc::chat_id::ChatId,
};

/// Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#editmessagereplymarkup)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Option<Message>)]
pub struct EditMessageReplyMarkupParams {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    /// Required if *inline\_message\_id* is not specified. Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    /// Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

// Divider: all content below this line will be preserved after code regen
