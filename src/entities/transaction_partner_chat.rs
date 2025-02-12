use serde::{Deserialize, Serialize};

use crate::entities::{chat::Chat, gift::Gift};

/// Describes a transaction with a chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#transactionpartnerchat)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartnerChat {
    /// Information about the chat
    pub chat: Box<Chat>,

    /// *Optional*. The gift sent to the chat by the bot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gift: Option<Gift>,
}

// Divider: all content below this line will be preserved after code regen
