use crate::entities::chat::Chat;
use crate::utils::deserialize_utils::deserialize_boxed;
use serde::{Deserialize, Serialize};

///The message was originally sent on behalf of a chat to a group chat.
///API Reference: [link](https://core.telegram.org/bots/api/#messageoriginchat)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageOriginChat {
    ///Date the message was sent originally in Unix time
    pub date: i64,

    ///Chat that sent the message originally
    #[serde(deserialize_with = "deserialize_boxed")]
    pub sender_chat: Box<Chat>,

    ///*Optional*. For messages originally sent by an anonymous chat administrator, original message author signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}
// Divider: all content below this line will be preserved after code regen
