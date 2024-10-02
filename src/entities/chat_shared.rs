use crate::entities::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// This object contains information about a chat that was shared with the bot using a [KeyboardButtonRequestChat](https://core.telegram.org/bots/api/#keyboardbuttonrequestchat) button.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatshared)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatShared {
    /// Identifier of the request
    pub request_id: i64,

    /// Identifier of the shared chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot may not have access to the chat and could be unable to use this identifier, unless the chat is already known to the bot by some other means.
    pub chat_id: i64,

    /// *Optional*. Title of the chat, if the title was requested by the bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// *Optional*. Username of the chat, if the username was requested by the bot and available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// *Optional*. Available sizes of the chat photo, if the photo was requested by the bot
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub photo: Vec<PhotoSize>,
}

// Divider: all content below this line will be preserved after code regen
