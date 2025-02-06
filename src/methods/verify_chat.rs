use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Verifies a chat [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#verifychat)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct VerifyChatParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_description: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
