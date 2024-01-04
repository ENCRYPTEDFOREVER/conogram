use crate::entities::chat::Chat;
use crate::utils::deserialize_utils::deserialize_boxed;
use serde::{Deserialize, Serialize};

///This object describes a message that was deleted or is otherwise inaccessible to the bot.
///API Reference: [link](https://core.telegram.org/bots/api/#inaccessiblemessage)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct InaccessibleMessage {
    ///Chat the message belonged to
    #[serde(deserialize_with = "deserialize_boxed")]
    pub chat: Box<Chat>,

    ///Unique message identifier inside the chat
    pub message_id: i64,

    ///Always 0. The field can be used to differentiate regular and inaccessible messages.
    pub date: i64,
}
// Divider: all content below this line will be preserved after code regen
