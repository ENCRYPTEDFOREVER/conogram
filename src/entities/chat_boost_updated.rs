use serde::{Deserialize, Serialize};

use crate::entities::{chat::Chat, chat_boost::ChatBoost};

/// This object represents a boost added to a chat or changed.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatboostupdated)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatBoostUpdated {
    /// Chat which was boosted
    pub chat: Box<Chat>,

    /// Information about the chat boost
    pub boost: ChatBoost,
}

// Divider: all content below this line will be preserved after code regen
