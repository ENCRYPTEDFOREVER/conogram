use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{
    inline_keyboard_markup::InlineKeyboardMarkup, misc::chat_id::ChatId, poll::Poll,
};

/// Use this method to stop a poll which was sent by the bot. On success, the stopped [Poll](https://core.telegram.org/bots/api/#poll) is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#stoppoll)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Poll)]
pub struct StopPollParams {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Identifier of the original message with the poll
    pub message_id: i64,

    /// A JSON-serialized object for a new message [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

// Divider: all content below this line will be preserved after code regen
