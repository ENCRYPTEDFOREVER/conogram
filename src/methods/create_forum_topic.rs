use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{forum_topic::ForumTopic, misc::chat_id::ChatId};

/// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. Returns information about the created topic as a [ForumTopic](https://core.telegram.org/bots/api/#forumtopic) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#createforumtopic)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = ForumTopic)]
pub struct CreateForumTopicParams {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatId,

    /// Topic name, 1-128 characters
    pub name: String,

    /// Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<i64>,

    /// Unique identifier of the custom emoji shown as the topic icon. Use [getForumTopicIconStickers](https://core.telegram.org/bots/api/#getforumtopiciconstickers) to get all allowed custom emoji identifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
