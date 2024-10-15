use serde::{Deserialize, Serialize};

use crate::entities::chat_boost::ChatBoost;

/// This object represents a list of boosts added to a chat by a user.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#userchatboosts)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserChatBoosts {
    /// The list of boosts added to the chat by the user
    pub boosts: Vec<ChatBoost>,
}

// Divider: all content below this line will be preserved after code regen
