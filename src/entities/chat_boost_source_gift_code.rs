use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// The boost was obtained by the creation of Telegram Premium gift codes to boost a chat. Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatboostsourcegiftcode)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatBoostSourceGiftCode {
    /// User for which the gift code was created
    pub user: User,
}

// Divider: all content below this line will be preserved after code regen
