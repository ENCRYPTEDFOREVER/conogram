use conogram_derives::Request;
use serde::Serialize;

use crate::{entities::input_rich_message::InputRichMessage, utils::deserialize_utils::is_false};

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

    /// Unique identifier of the message draft; must be non-zero. Changes to drafts with the same identifier are animated. Otherwise, the draft is replaced without animation.
    pub draft_id: i64,

    /// The partial message to be streamed. Direct upload of new files and explicit upload of files by a URL isn't supported.
    pub rich_message: InputRichMessage,

    /// Pass *True* to show the user a button to stop further drafts. The bot will receive an [Update](https://core.telegram.org/bots/api/#update) “stopped\_message\_generation” if the user presses the button.
    #[serde(skip_serializing_if = "is_false")]
    pub can_stop: bool,

    /// Pass *True* to keep the draft in the chat when the button is pressed. The draft will still disappear after a short time or if the bot sends a message. To fully preserve the partial draft, the bot should send it as a new message.
    #[serde(skip_serializing_if = "is_false")]
    pub keep_on_stop: bool,
}

// Divider: all content below this line will be preserved after code regen
