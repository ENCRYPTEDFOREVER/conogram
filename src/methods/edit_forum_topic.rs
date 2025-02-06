use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#editforumtopic)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct EditForumTopicParams {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatId,

    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,

    /// New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// New unique identifier of the custom emoji shown as the topic icon. Use [getForumTopicIconStickers](https://core.telegram.org/bots/api/#getforumtopiciconstickers) to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
