use conogram_derives::Request;
use serde::Serialize;

use crate::entities::message_entity::MessageEntity;

/// Use this method to stream a partial message to a user while the message is being generated. Note that the streamed draft is ephemeral and acts as a temporary 30-second preview - once the output is finalized, you **must** call [sendMessage](https://core.telegram.org/bots/api/#sendmessage) with the complete message to persist it in the user's chat. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#sendmessagedraft)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SendMessageDraftParams {
    /// Unique identifier for the target private chat
    pub chat_id: i64,

    /// Unique identifier for the target message thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// Unique identifier of the message draft; must be non-zero. Changes of drafts with the same identifier are animated.
    pub draft_id: i64,

    /// Text of the message to be sent, 0-4096 characters after entities parsing. Pass an empty text to show a “Thinking…” placeholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<MessageEntity>,
}

// Divider: all content below this line will be preserved after code regen
