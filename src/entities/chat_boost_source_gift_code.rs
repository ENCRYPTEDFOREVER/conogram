use crate::entities::user::User;
use serde::{Deserialize, Serialize};

///The boost was obtained by the creation of Telegram Premium gift codes to boost a chat. Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
///API Reference: [link](https://core.telegram.org/bots/api/#chatboostsourcegiftcode)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatBoostSourceGiftCode {
    ///User for which the gift code was created
    pub user: User,
}
// Divider: all content below this line will be preserved after code regen
