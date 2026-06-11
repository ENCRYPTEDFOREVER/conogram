use conogram_derives::Request;
use serde::Serialize;

/// Use this method to process a received chat join request query. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#answerchatjoinrequestquery)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct AnswerChatJoinRequestQueryParams {
    /// Unique identifier of the join request query
    pub chat_join_request_query_id: String,

    /// Result of the query. Must be either “approve” to allow the user to join the chat, “decline” to disallow the user to join the chat, or “queue” to leave the decision to other administrators.
    pub result: AnswerChatJoinRequestQueryResult,
}

/// Result of the query. Must be either “approve” to allow the user to join the chat, “decline” to disallow the user to join the chat, or “queue” to leave the decision to other administrators.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub enum AnswerChatJoinRequestQueryResult {
    /// `approve`
    #[default]
    #[serde(rename = "approve")]
    Approve,

    /// `decline`
    #[serde(rename = "decline")]
    Decline,

    /// `queue`
    #[serde(rename = "queue")]
    Queue,
}

// Divider: all content below this line will be preserved after code regen
