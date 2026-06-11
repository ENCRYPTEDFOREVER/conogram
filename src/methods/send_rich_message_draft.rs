use conogram_derives::Request;
use serde::Serialize;

use crate::entities::input_rich_message::InputRichMessage;

/// Use this method to stream a partial rich message to a user while the message is being generated. Note that the streamed draft is ephemeral and acts as a temporary 30-second preview - once the output is finalized, you **must** call [sendRichMessage](https://core.telegram.org/bots/api/#sendrichmessage) with the complete message to persist it in the user's chat. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#sendrichmessagedraft)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SendRichMessageDraftParams {
    /// Unique identifier for the target private chat
    pub chat_id: i64,

    /// Unique identifier for the target message thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// Unique identifier of the message draft; must be non-zero. Changes to drafts with the same identifier are animated.
    pub draft_id: i64,

    /// The partial message to be streamed
    pub rich_message: InputRichMessage,
}

// Divider: all content below this line will be preserved after code regen
