use crate::entities::inaccessible_message::InaccessibleMessage;
use crate::entities::message::Message;
use serde::{Deserialize, Serialize};

///This object describes a message that can be inaccessible to the bot. It can be one of
///
///* [Message](https://core.telegram.org/bots/api/#message)
///* [InaccessibleMessage](https://core.telegram.org/bots/api/#inaccessiblemessage)
///
///API Reference: [link](https://core.telegram.org/bots/api/#maybeinaccessiblemessage)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    Message(Message),
    InaccessibleMessage(InaccessibleMessage),
}
impl Default for MaybeInaccessibleMessage {
    fn default() -> Self {
        Self::Message(Message::default())
    }
}
impl From<Message> for MaybeInaccessibleMessage {
    fn from(value: Message) -> Self {
        Self::Message(value)
    }
}

impl From<InaccessibleMessage> for MaybeInaccessibleMessage {
    fn from(value: InaccessibleMessage) -> Self {
        Self::InaccessibleMessage(value)
    }
}
// Divider: all content below this line will be preserved after code regen
