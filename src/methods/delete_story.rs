use conogram_derives::Request;
use serde::Serialize;

/// Deletes a story previously posted by the bot on behalf of a managed business account. Requires the *can\_manage\_stories* business bot right. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#deletestory)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct DeleteStoryParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// Unique identifier of the story to delete
    pub story_id: i64,
}

// Divider: all content below this line will be preserved after code regen
