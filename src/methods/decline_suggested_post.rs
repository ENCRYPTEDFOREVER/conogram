use conogram_derives::Request;
use serde::Serialize;

/// Use this method to decline a suggested post in a direct messages chat. The bot must have the 'can\_manage\_direct\_messages' administrator right in the corresponding channel chat. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#declinesuggestedpost)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct DeclineSuggestedPostParams {
    /// Unique identifier for the target direct messages chat
    pub chat_id: i64,

    /// Identifier of a suggested post message to decline
    pub message_id: i64,

    /// Comment for the creator of the suggested post; 0-128 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
