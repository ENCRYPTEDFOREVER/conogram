use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// The boost was obtained by subscribing to Telegram Premium or by gifting a Telegram Premium subscription to another user.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatboostsourcepremium)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatBoostSourcePremium {
    /// User that boosted the chat
    pub user: User,
}

// Divider: all content below this line will be preserved after code regen
