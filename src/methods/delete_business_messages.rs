use conogram_derives::Request;
use serde::Serialize;

/// Delete messages on behalf of a business account. Requires the *can\_delete\_outgoing\_messages* business bot right to delete messages sent by the bot itself, or the *can\_delete\_all\_messages* business bot right to delete any message. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#deletebusinessmessages)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct DeleteBusinessMessagesParams {
    /// Unique identifier of the business connection on behalf of which to delete the messages
    pub business_connection_id: String,

    /// A JSON-serialized list of 1-100 identifiers of messages to delete. All messages must be from the same chat. See [deleteMessage](https://core.telegram.org/bots/api/#deletemessage) for limitations on which messages can be deleted
    pub message_ids: Vec<i64>,
}

// Divider: all content below this line will be preserved after code regen
