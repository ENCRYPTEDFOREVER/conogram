use crate::entities::user::User;
use serde::{Deserialize, Serialize};

///The message was originally sent by a known user.
///API Reference: [link](https://core.telegram.org/bots/api/#messageoriginuser)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageOriginUser {
    ///Date the message was sent originally in Unix time
    pub date: i64,

    ///User that sent the message originally
    pub sender_user: User,
}
// Divider: all content below this line will be preserved after code regen
