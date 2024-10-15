use serde::{Deserialize, Serialize};

use crate::utils::deserialize_utils::is_false;

/// Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice [privacy mode](https://core.telegram.org/bots/features#privacy-mode). Not supported in channels and for messages sent on behalf of a Telegram Business account.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#forcereply)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
    pub force_reply: bool,

    /// *Optional*. The placeholder to be shown in the input field when the reply is active; 1-64 characters
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,

    /// *Optional*. Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the *text* of the [Message](https://core.telegram.org/bots/api/#message) object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.
    #[serde(default, skip_serializing_if = "is_false")]
    pub selective: bool,
}

// Divider: all content below this line will be preserved after code regen
