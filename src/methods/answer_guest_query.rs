use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{
    inline_query_result::InlineQueryResult, sent_guest_message::SentGuestMessage,
};

/// Use this method to reply to a received guest message. On success, a [SentGuestMessage](https://core.telegram.org/bots/api/#sentguestmessage) object is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#answerguestquery)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = SentGuestMessage)]
pub struct AnswerGuestQueryParams {
    /// Unique identifier for the query to be answered
    pub guest_query_id: String,

    /// A JSON-serialized object describing the message to be sent
    pub result: InlineQueryResult,
}

// Divider: all content below this line will be preserved after code regen
