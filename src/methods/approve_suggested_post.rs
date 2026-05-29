use conogram_derives::Request;
use serde::Serialize;

/// Use this method to approve a suggested post in a direct messages chat. The bot must have the 'can\_post\_messages' administrator right in the corresponding channel chat. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#approvesuggestedpost)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct ApproveSuggestedPostParams {
    /// Unique identifier for the target direct messages chat
    pub chat_id: i64,

    /// Identifier of a suggested post message to approve
    pub message_id: i64,

    /// Point in time (Unix timestamp) when the post is expected to be published; omit if the date has already been specified when the suggested post was created. If specified, then the date must be not more than 2678400 seconds (30 days) in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
