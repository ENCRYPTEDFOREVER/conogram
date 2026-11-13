use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{message_entity::MessageEntity, misc::chat_id::ChatId};

/// Verifies a chat [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#verifychat)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct VerifyChatParams {
    /// Unique identifier for the target chat or username of the target bot, supergroup, or channel in the format `@username`. Channel direct messages chats can't be verified.
    pub chat_id: ChatId,

    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_description: Option<String>,

    /// Mode for parsing entities in the custom description. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Only “text\_link” entities are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_description_parse_mode: Option<String>,

    /// A JSON-serialized list of special entities that appear in the custom description, which can be specified instead of *custom\_description\_parse\_mode*. Only “text\_link” entities are supported.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub custom_description_entities: Vec<MessageEntity>,
}

// Divider: all content below this line will be preserved after code regen
