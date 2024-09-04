use serde::{Deserialize, Serialize};

///The message was originally sent by an unknown user.
///
///API Reference: [link](https://core.telegram.org/bots/api/#messageoriginhiddenuser)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageOriginHiddenUser {
    ///Date the message was sent originally in Unix time
    pub date: i64,

    ///Name of the user that sent the message originally
    pub sender_user_name: String,
}
// Divider: all content below this line will be preserved after code regen
