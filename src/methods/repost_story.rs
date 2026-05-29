use conogram_derives::Request;
use serde::Serialize;

use crate::{entities::story::Story, utils::deserialize_utils::is_false};

/// Reposts a story on behalf of a business account from another business account. Both business accounts must be managed by the same bot, and the story on the source account must have been posted (or reposted) by the bot. Requires the *can\_manage\_stories* business bot right for both business accounts. Returns [Story](https://core.telegram.org/bots/api/#story) on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#repoststory)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Story)]
pub struct RepostStoryParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// Unique identifier of the chat which posted the story that should be reposted
    pub from_chat_id: i64,

    /// Unique identifier of the story that should be reposted
    pub from_story_id: i64,

    /// Period after which the story is moved to the archive, in seconds; must be one of `6 * 3600`, `12 * 3600`, `86400`, or `2 * 86400`
    pub active_period: i64,

    /// Pass *True* to keep the story accessible after it expires
    #[serde(skip_serializing_if = "is_false")]
    pub post_to_chat_page: bool,

    /// Pass *True* if the content of the story must be protected from forwarding and screenshotting
    #[serde(skip_serializing_if = "is_false")]
    pub protect_content: bool,
}

// Divider: all content below this line will be preserved after code regen
